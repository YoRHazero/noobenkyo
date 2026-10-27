"""Puts the executor's code on the GPU machine. ai-invoker sends this file to
`python -` on stdin with one line appended: a call to `install` with the
code's hash and the code itself (base64 of a JSON object mapping relative
file paths, "/" between directories, to contents). Prints the absolute path
of the code's directory."""

import base64
import json
import os
import shutil
import tempfile

ROOT = os.path.expanduser("~/.cache/noobenkyo-executor")


def install(digest, bundle):
    target = os.path.join(ROOT, digest)
    if not os.path.isdir(target):
        os.makedirs(ROOT, exist_ok=True)
        staging = tempfile.mkdtemp(prefix=f".{digest}-", dir=ROOT)
        for name, text in json.loads(base64.b64decode(bundle)).items():
            path = os.path.join(staging, *name.split("/"))
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as file:
                file.write(text)
        try:
            os.rename(staging, target)
        except OSError:
            # Another start put the same version there first.
            shutil.rmtree(staging, ignore_errors=True)
        # Executors running from an old version have their code loaded already.
        for entry in os.listdir(ROOT):
            if entry != digest:
                shutil.rmtree(os.path.join(ROOT, entry), ignore_errors=True)
    print(target, flush=True)
