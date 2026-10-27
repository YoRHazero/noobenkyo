"""Runs llama-server as a child process listening on 127.0.0.1 only, and
passes `llm_chat` to its streaming chat endpoint. The standard library only,
so any python runs it."""

import collections
import ctypes
import http.client
import json
import os
import signal
import socket
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request

from protocol import Cancelled, RequestError

START_FIELDS = {"server", "path", "args"}
PROGRESS_EVERY = 64
PR_SET_PDEATHSIG = 1

# Looked up before any fork: the child may only call it, not load it.
_prctl = ctypes.CDLL(None, use_errno=True).prctl


def _die_with_parent():
    _prctl(PR_SET_PDEATHSIG, signal.SIGKILL)


class Runtime:
    def __init__(self, start, children):
        if not os.path.isfile(start["server"]):
            raise RuntimeError(f"llama-server not found: {start['server']}")
        if not os.path.isfile(start["path"]):
            raise RuntimeError(f"model path not found: {start['path']}")
        self._port = _free_port()
        self._log = collections.deque(maxlen=20)
        self._server = subprocess.Popen(
            [
                start["server"],
                "-m", start["path"],
                "--host", "127.0.0.1",
                "--port", str(self._port),
                "-np", "1",
                *start["args"],
            ],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            preexec_fn=_die_with_parent,
        )
        children.append(self._server)
        threading.Thread(target=self._forward_log, daemon=True).start()
        self._wait_until_healthy()
        self._build = self._get_json("/props").get("build_info", "unknown")

    def versions(self):
        return {"llama_server": self._build}

    def handle(self, job):
        request = job.request
        if request["type"] != "llm_chat":
            raise RequestError(f"llama_cpp does not take {request['type']}")
        body = dict(request["body"], stream=True)
        connection = http.client.HTTPConnection("127.0.0.1", self._port)
        job.on_cancel(lambda: _hang_up(connection))
        try:
            connection.request(
                "POST",
                "/v1/chat/completions",
                body=json.dumps(body).encode(),
                headers={"Content-Type": "application/json"},
            )
            if job.cancelled:
                raise Cancelled()
            response = connection.getresponse()
            if response.status != 200:
                text = response.read().decode(errors="replace")
                self._check_alive()
                raise RequestError(f"llama-server rejected the request: {response.status} {text}")
            return self._read_stream(response, job)
        except (OSError, http.client.HTTPException):
            if job.cancelled:
                raise Cancelled() from None
            self._check_alive()
            raise
        finally:
            job.on_cancel(None)
            connection.close()

    def _read_stream(self, response, job):
        pieces = []
        counted = 0
        finish = None
        predicted = None
        for raw in response:
            line = raw.strip()
            if not line.startswith(b"data:"):
                continue
            data = line[len(b"data:"):].strip()
            if data == b"[DONE]":
                break
            chunk = json.loads(data)
            if "error" in chunk:
                raise RequestError(f"llama-server rejected the request: {chunk['error']}")
            for choice in chunk.get("choices", []):
                delta = choice.get("delta") or {}
                if delta.get("content"):
                    pieces.append(delta["content"])
                if delta.get("content") or delta.get("reasoning_content"):
                    counted += 1
                    if counted % PROGRESS_EVERY == 0:
                        job.progress(counted)
                if choice.get("finish_reason"):
                    finish = choice["finish_reason"]
            predicted = (chunk.get("timings") or {}).get("predicted_n", predicted)
        if job.cancelled:
            raise Cancelled()
        if finish is None:
            self._check_alive()
            raise RequestError("llama-server ended the stream without finishing")
        if finish not in ("stop", "length"):
            raise RequestError(f"llama-server finished with {finish!r}")
        tokens = predicted if predicted is not None else counted
        return {"text": "".join(pieces), "tokens": tokens, "finish": finish}

    def _wait_until_healthy(self):
        while True:
            self._check_alive()
            try:
                with urllib.request.urlopen(self._url("/health"), timeout=2) as response:
                    if response.status == 200:
                        return
            except (urllib.error.URLError, OSError, http.client.HTTPException):
                pass
            time.sleep(0.25)

    def _get_json(self, path):
        with urllib.request.urlopen(self._url(path), timeout=10) as response:
            return json.load(response)

    def _url(self, path):
        return f"http://127.0.0.1:{self._port}{path}"

    def _check_alive(self):
        code = self._server.poll()
        if code is not None:
            time.sleep(0.2)  # let the log thread catch the last lines
            last = "\n".join(list(self._log)[-5:])
            raise RuntimeError(f"llama-server exited with code {code}: {last}")

    def _forward_log(self):
        for raw in self._server.stdout:
            line = raw.decode(errors="replace").rstrip()
            self._log.append(line)
            print(f"llama-server: {line}", file=sys.stderr, flush=True)


def _free_port():
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        return probe.getsockname()[1]


def _hang_up(connection):
    """Closing the connection is how llama-server learns to stop."""
    if connection.sock is not None:
        try:
            connection.sock.shutdown(socket.SHUT_RDWR)
        except OSError:
            pass
