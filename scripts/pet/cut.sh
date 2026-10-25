#!/usr/bin/env bash
# Cuts every pet sheet in art/pet into the frames the app uses, in
# web/src/features/pet/sprites. Each line below is how one sheet was drawn:
# its grid, how many cells are frames, which cell is the standing pose, how
# the frames line up. The same sheets, grid.py and libraries (requirements.txt)
# give the same bytes, so a run changes only the sheets that were redrawn.
#
#   bash scripts/pet/cut.sh            cut into sprites/
#   bash scripts/pet/cut.sh --check    cut into a temporary directory and list
#                                      what would change; sprites/ is untouched
#
# PYTHON picks the interpreter (default python3).
set -euo pipefail
shopt -s nullglob

root="$(cd "$(dirname "$0")/../.." && pwd)"
art="$root/art/pet"
sprites="$root/web/src/features/pet/sprites"
python="${PYTHON:-python3}"

check=false
if [[ "${1:-}" == --check ]]; then
  check=true
fi

out="$sprites"
if $check; then
  out="$(mktemp -d)"
  trap 'rm -rf "$out"' EXIT
fi
mkdir -p "$out"

# cut <sheet> <rows> <cols> <cells> [grid.py options]
cut() {
  local sheet="$1"
  shift
  "$python" "$root/scripts/pet/grid.py" "$art/$sheet.png" "$1" "$2" "$3" "$out" "$sheet" "${@:4}"
}

cut standing 1 1 1
cut dragged 2 3 5 --drop 1 --align top
cut falling 2 3 5 --drop 1
cut duck-sit 2 3 6 --size 6
cut yawn 2 3 6
cut sway 2 3 6
cut sleep 2 3 5 --align left
cut request 2 4 7 --size 7 --part 8:paper-ball
cut phone 2 3 5
cut troubled 2 3 6

if $check; then
  changed=0
  for file in "$out"/*; do
    name="$(basename "$file")"
    if [[ ! -e "$sprites/$name" ]]; then
      echo "new: $name"
      changed=1
    elif ! cmp -s "$file" "$sprites/$name"; then
      echo "would change: $name"
      changed=1
    fi
  done
  for file in "$sprites"/*; do
    name="$(basename "$file")"
    if [[ ! -e "$out/$name" ]]; then
      echo "no longer cut: $name"
      changed=1
    fi
  done
  if [[ $changed == 0 ]]; then
    echo "sprites/ is up to date"
  fi
  exit $changed
fi
