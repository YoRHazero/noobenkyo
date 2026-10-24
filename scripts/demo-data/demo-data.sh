#!/usr/bin/env bash
# Demo data for the listening pages: 星见町 with four characters and two
# episodes, every reference and line voiced by qwen-tts on the GPU machine.
# Writes into the app's data directory; does nothing if 星见町 is there.
#
#   bash scripts/demo-data/demo-data.sh [data dir]
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
data="${1:-$HOME/Library/Application Support/com.noobenkyo.desktop}"
host="yorha@yorha.local"
remote_dir="noobenkyo-demo"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# Runs a bash script inside WSL; the remote shell is cmd, so it goes on stdin.
remote() { ssh "$host" 'wsl -e bash -s'; }

# Copies local files into the remote work directory, as base64 so nothing mangles the bytes.
upload() {
  local payload
  payload="$(tar czf - -C "$1" "${@:2}" | base64)"
  remote <<SH
mkdir -p ~/$remote_dir && cd ~/$remote_dir
base64 -d <<'B64' | tar xzf -
$payload
B64
SH
}

# Copies a remote directory's files into a local one.
download() {
  mkdir -p "$2"
  remote <<SH | base64 -d | tar xzf - -C "$2"
cd ~/$remote_dir/$1 && tar czf - . | base64 -w0
SH
}

run_python() {
  remote <<SH
cd ~/$remote_dir && ~/opt/qwen-tts/.venv/bin/python synthesize.py $* 2>&1 | grep -v -i 'flash-attn\|^\*\*\*\*\|^ *$'
SH
}

demo() { (cd "$here/../.." && cargo run -q -p listening --example demo_data -- "$@"); }

echo "== references (VoiceDesign)"
upload "$here" cast.json synthesize.py
run_python references cast.json refs
download refs "$work/refs"

echo "== characters, world, episodes"
demo seed "$data" "$here/cast.json" "$work/refs" >"$work/jobs.json"
if [ "$(tr -d '[:space:]' <"$work/jobs.json")" = "[]" ]; then
  exit 0
fi

echo "== lines (Base)"
cp "$work/jobs.json" "$here/../../target/demo-jobs.json" 2>/dev/null || true
upload "$work" jobs.json
run_python lines cast.json jobs.json refs audio
download audio "$work/audio"

echo "== saving audio"
demo save "$data" "$work/jobs.json" "$work/audio"
