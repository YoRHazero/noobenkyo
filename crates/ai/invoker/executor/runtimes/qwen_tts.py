"""Loads one qwen-tts model in this process and does `tts_clone` or
`tts_design` with it, whichever the model is for. Needs torch, qwen_tts and
soundfile, so it only runs in qwen-tts's environment. A sentence cannot be
stopped halfway: a cancel takes effect once it is done."""

import base64
import importlib.metadata
import io
import os

import soundfile
import torch
from qwen_tts import Qwen3TTSModel

from protocol import RequestError

START_FIELDS = {"path"}


class Runtime:
    def __init__(self, start, children):
        if not os.path.isdir(start["path"]):
            raise RuntimeError(f"model path not found: {start['path']}")
        self._model = Qwen3TTSModel.from_pretrained(
            start["path"], device_map="cuda:0", dtype=torch.bfloat16
        )

    def versions(self):
        return {
            "torch": torch.__version__,
            "qwen_tts": importlib.metadata.version("qwen-tts"),
        }

    def handle(self, job):
        request = job.request
        if request["type"] == "tts_clone":
            wavs, rate = self._clone(request)
        elif request["type"] == "tts_design":
            wavs, rate = self._design(request)
        else:
            raise RequestError(f"qwen_tts does not take {request['type']}")
        # soundfile's default for WAV: 16-bit integers.
        wav = io.BytesIO()
        soundfile.write(wav, wavs[0], rate, format="WAV")
        return {"wav": base64.b64encode(wav.getvalue()).decode("ascii")}

    def _clone(self, request):
        try:
            data = base64.b64decode(request["ref_audio"], validate=True)
            audio, rate = soundfile.read(io.BytesIO(data), dtype="float32", always_2d=False)
        except Exception as error:
            raise RequestError(f"cannot read ref_audio: {error}") from None
        if audio.ndim > 1:
            audio = audio.mean(axis=-1)
        # Made anew each time: 0.05 s once warm.
        prompt = self._model.create_voice_clone_prompt(
            ref_audio=(audio, rate), ref_text=request["ref_text"]
        )
        return self._model.generate_voice_clone(
            text=request["text"], language=request["language"], voice_clone_prompt=prompt
        )

    def _design(self, request):
        if request.get("seed") is not None:
            torch.manual_seed(request["seed"])
        return self._model.generate_voice_design(
            text=request["text"], instruct=request["instruct"], language=request["language"]
        )
