"""Demo voices for listening, run on the GPU machine with qwen-tts.

    synthesize.py references <cast.json> <out_dir>
        Each character's references, ja and zh, made by VoiceDesign from its
        prompt with the same seed, as <name>-<language>.wav.
    synthesize.py lines <cast.json> <jobs.json> <refs_dir> <out_dir>
        Each job (a line in one language) cloned by Base from its voice's
        reference in that language, as <line>-<language>.wav.

Only one model fits in the GPU at a time, so each mode loads one.
"""

import json
import os
import sys

import soundfile as sf
import torch
from qwen_tts import Qwen3TTSModel

MODELS = os.path.expanduser("~/models")
LANGUAGES = {"ja": "Japanese", "zh": "Chinese"}


def load(name):
    return Qwen3TTSModel.from_pretrained(
        os.path.join(MODELS, name), device_map="cuda:0", dtype=torch.bfloat16
    )


def references(cast_path, out_dir):
    cast = json.load(open(cast_path))
    designer = load("Qwen3-TTS-12Hz-1.7B-VoiceDesign")
    for character in cast["characters"]:
        for language, text in character["references"].items():
            torch.manual_seed(character["seed"])
            wavs, rate = designer.generate_voice_design(
                text=text, instruct=character["prompt"], language=LANGUAGES[language]
            )
            name = f"{character['name']}-{language}.wav"
            sf.write(os.path.join(out_dir, name), wavs[0], rate)
            print(name, flush=True)


def lines(cast_path, jobs_path, refs_dir, out_dir):
    cast = json.load(open(cast_path))
    texts = {c["name"]: c["references"] for c in cast["characters"]}
    cloner = load("Qwen3-TTS-12Hz-1.7B-Base")
    prompts = {}
    for index, job in enumerate(json.load(open(jobs_path))):
        voice, language = job["voice"], job["language"]
        key = (voice, language)
        if key not in prompts:
            prompts[key] = cloner.create_voice_clone_prompt(
                ref_audio=os.path.join(refs_dir, f"{voice}-{language}.wav"),
                ref_text=texts[voice][language],
            )
        torch.manual_seed(1000 + index)
        wavs, rate = cloner.generate_voice_clone(
            text=job["text"], language=LANGUAGES[language], voice_clone_prompt=prompts[key]
        )
        name = f"{job['line']}-{language}.wav"
        sf.write(os.path.join(out_dir, name), wavs[0], rate)
        print(name, flush=True)


if __name__ == "__main__":
    mode, *args = sys.argv[1:]
    os.makedirs(args[-1], exist_ok=True)
    {"references": references, "lines": lines}[mode](*args)
