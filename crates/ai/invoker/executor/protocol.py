"""The executor's side of the protocol: one JSON message per line, stdin from
the app and stdout back. The real stdout belongs to the protocol alone;
anything else that writes there (libraries, child processes) is moved to
stderr, which the app reads as a log. Also how a request can end besides
`done`: cancelled, or failed while the executor stays up."""

import json
import os
import sys
import threading


class ProtocolError(Exception):
    """A message the two sides should never disagree on: always a bug."""


class Cancelled(Exception):
    """The app cancelled the request and the work on it has stopped."""


class RequestError(Exception):
    """The request failed; the executor is fine and takes the next one."""


# The fields of each message from the app besides "type": (required, optional).
REQUESTS = {
    "llm_chat": ({"id", "body"}, set()),
    "tts_clone": ({"id", "text", "language", "ref_audio", "ref_text"}, set()),
    "tts_design": ({"id", "text", "language", "instruct"}, {"seed"}),
}
CONTROLS = {
    "cancel": ({"id"}, set()),
    "ping": (set(), set()),
}


def check_fields(message, required, optional=frozenset()):
    fields = set(message) - {"type"}
    missing = required - fields
    unknown = fields - required - set(optional)
    if missing or unknown:
        raise ProtocolError(
            f"{message['type']}: missing {sorted(missing)}, unknown {sorted(unknown)}"
        )


class Channel:
    """Must be made before anything else writes: it moves file descriptor 1
    to stderr and keeps a private copy of the real stdout."""

    def __init__(self):
        protocol_fd = os.dup(1)
        os.dup2(2, 1)
        sys.stdout = sys.stderr
        self._out = os.fdopen(protocol_fd, "wb")
        self._in = sys.stdin.buffer
        self._lock = threading.Lock()

    def send(self, kind, **fields):
        line = json.dumps({"type": kind, **fields}, separators=(",", ":"))
        with self._lock:
            self._out.write(line.encode("ascii") + b"\n")
            self._out.flush()

    def receive(self):
        """The next message, or None once the app has closed stdin."""
        line = self._in.readline()
        if not line:
            return None
        try:
            message = json.loads(line)
        except ValueError as error:
            raise ProtocolError(f"not JSON: {error}") from None
        if not isinstance(message, dict) or not isinstance(message.get("type"), str):
            raise ProtocolError(f"not a message: {line[:200]!r}")
        return message
