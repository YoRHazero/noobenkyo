"""Looks at a GPU machine for noobenkyo's model page. ai-invoker sends this
file to `python -` on stdin with one line appended: a call to `probe` with
base64 of a JSON object holding `models`, the models directory, and `files`,
the paths to check. Prints one line of JSON and loads no model. Standard
library only, so any python3 runs it. See dev_docs/Invoker.md「探查」."""

import base64
import json
import os


def probe(encoded):
    look = json.loads(base64.b64decode(encoded))
    models = look["models"]
    report = {
        "models_dir": os.path.isdir(models),
        "files": {path: os.path.isfile(path) for path in look["files"]},
        "tts": tts_models(models),
        "gguf": gguf_models(models),
    }
    print(json.dumps(report), flush=True)


def tts_models(models):
    """qwen-tts models: the directories one level down whose config.json
    says qwen3_tts. Its tts_model_type says which call each takes."""
    if not os.path.isdir(models):
        return []
    found = []
    for name in sorted(os.listdir(models)):
        path = os.path.join(models, name)
        config = read_config(os.path.join(path, "config.json"))
        if config.get("model_type") == "qwen3_tts":
            found.append({"name": name, "path": path, "type": config.get("tts_model_type")})
    return found


def read_config(path):
    try:
        with open(path, encoding="utf-8") as file:
            config = json.load(file)
    except (OSError, ValueError):
        return {}
    return config if isinstance(config, dict) else {}


def gguf_models(models):
    """GGUF files anywhere below, each a model named after its file. Hidden
    directories, such as a download cache, are left out."""
    found = []
    for root, dirs, files in os.walk(models):
        dirs[:] = sorted(d for d in dirs if not d.startswith("."))
        for file in sorted(files):
            if file.endswith(".gguf"):
                found.append({"name": file[: -len(".gguf")], "path": os.path.join(root, file)})
    return found
