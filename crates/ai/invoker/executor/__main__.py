"""The executor, started by ai-invoker on the GPU machine as
`python <this directory>`. Reads `start`, loads the one model it names, says
`ready`, then does one request at a time until stdin closes. A thread reads
stdin all along, so cancels and pings get through while a request runs.
See dev_docs/Invoker.md."""

import importlib
import os
import platform
import queue
import sys
import threading
import time
import traceback

from protocol import (
    CONTROLS,
    REQUESTS,
    Channel,
    ProtocolError,
    RequestError,
    check_fields,
)

# The module that runs each runtime. Only the one asked for is imported:
# qwen-tts's needs torch, which only qwen-tts's environment has.
RUNTIMES = {"llama_cpp": "runtimes.llama_cpp", "qwen_tts": "runtimes.qwen_tts"}


def main():
    channel = Channel()
    start = channel.receive()
    if start is None:
        return
    children = []
    try:
        if start["type"] != "start":
            raise ProtocolError(f"the first message is {start['type']}, not start")
        if start.get("runtime") not in RUNTIMES:
            raise ProtocolError(f"unknown runtime {start.get('runtime')!r}")
        inbox = Inbox(channel, start["heartbeat_timeout_secs"], children)
        inbox.listen()
        module = importlib.import_module(RUNTIMES[start["runtime"]])
        check_fields(start, {"runtime", "heartbeat_timeout_secs"} | module.START_FIELDS)
        runtime = module.Runtime(start, children)
    except Exception as error:
        traceback.print_exc()
        channel.send("error", id=None, message=describe(error))
        stop(children, 1)
    versions = {"python": platform.python_version(), **runtime.versions()}
    channel.send("ready", versions=versions)
    while True:
        job = inbox.next_job()
        if job is not None:
            work(channel, inbox, runtime, job)


def work(channel, inbox, runtime, job):
    try:
        fields = runtime.handle(job)
        if job.cancelled:
            channel.send("cancelled", id=job.id)
        else:
            channel.send("done", id=job.id, **fields)
    except Exception as error:
        if job.cancelled:
            channel.send("cancelled", id=job.id)
        elif isinstance(error, RequestError):
            channel.send("error", id=job.id, message=str(error))
        else:
            # Anything unforeseen may have left the model in a bad state.
            traceback.print_exc()
            channel.send("error", id=job.id, message=describe(error))
            stop(inbox.children, 1)
    finally:
        inbox.finish(job)


def describe(error):
    if isinstance(error, (RequestError, ProtocolError, RuntimeError)):
        return str(error)
    return f"{type(error).__name__}: {error}"


def stop(children, code):
    """Leaves at once, taking the child processes along."""
    for child in children:
        try:
            child.kill()
        except OSError:
            pass
    sys.stderr.flush()
    os._exit(code)


class Job:
    """One request being worked on. The stdin thread may cancel it any time;
    the runtime checks `cancelled` and can register what to do on cancel."""

    def __init__(self, request, channel):
        self.request = request
        self.id = request["id"]
        self.cancelled = False
        self._channel = channel
        self._lock = threading.Lock()
        self._on_cancel = None

    def progress(self, tokens):
        self._channel.send("progress", id=self.id, tokens=tokens)

    def on_cancel(self, action):
        """Runs `action` when cancelled, or now if already cancelled. None clears it."""
        with self._lock:
            self._on_cancel = action
            already = self.cancelled
        if already and action is not None:
            action()

    def cancel(self):
        with self._lock:
            self.cancelled = True
            action = self._on_cancel
        if action is not None:
            action()



class Inbox:
    """Reads stdin on its own thread and keeps the requests for the main
    thread. Leaves when stdin closes or the app goes quiet for too long."""

    def __init__(self, channel, heartbeat_timeout, children):
        self.children = children
        self._channel = channel
        self._timeout = heartbeat_timeout
        self._requests = queue.Queue()
        self._lock = threading.Lock()
        self._cancelled = set()
        self._current = None
        self._last_heard = time.monotonic()

    def listen(self):
        threading.Thread(target=self._read, daemon=True).start()
        threading.Thread(target=self._watch, daemon=True).start()

    def next_job(self):
        """The next request as a Job; None if it was cancelled while waiting."""
        request = self._requests.get()
        with self._lock:
            if request["id"] in self._cancelled:
                self._cancelled.discard(request["id"])
                job = None
            else:
                job = self._current = Job(request, self._channel)
        if job is None:
            self._channel.send("cancelled", id=request["id"])
        return job

    def finish(self, job):
        with self._lock:
            self._current = None

    def _read(self):
        message = None
        try:
            while True:
                message = self._channel.receive()
                if message is None:
                    stop(self.children, 0)
                self._last_heard = time.monotonic()
                self._take(message)
        except Exception as error:
            traceback.print_exc()
            request_id = message.get("id") if isinstance(message, dict) else None
            self._channel.send("error", id=request_id, message=describe(error))
            stop(self.children, 1)

    def _take(self, message):
        kind = message["type"]
        if kind in REQUESTS:
            check_fields(message, *REQUESTS[kind])
            self._requests.put(message)
        elif kind in CONTROLS:
            check_fields(message, *CONTROLS[kind])
            if kind == "cancel":
                self._cancel(message["id"])
        else:
            raise ProtocolError(f"unknown message type {kind!r}")

    def _cancel(self, request_id):
        with self._lock:
            job = self._current
            if job is None or job.id != request_id:
                job = None
                self._cancelled.add(request_id)
        if job is not None:
            job.cancel()

    def _watch(self):
        while True:
            time.sleep(1)
            quiet = time.monotonic() - self._last_heard
            if quiet > self._timeout:
                print(f"executor: nothing from the app for {quiet:.0f} s, leaving", file=sys.stderr)
                stop(self.children, 1)


main()
