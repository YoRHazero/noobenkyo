#!/usr/bin/env bash
# Regenerates the SeaORM entities from the migrations.
# Migrations are the only source of the schema; entities are never edited by hand.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

db="$tmp/dictionary.db"  # created by the migrate example in $tmp
cargo run --quiet --manifest-path "$root/Cargo.toml" -p dictionary --example migrate -- "$tmp"

generate() { # <tables> <output dir>
  rm -rf "$2"
  sea-orm-cli generate entity \
    --database-url "sqlite://$db" \
    --tables "$1" \
    --output-dir "$2" \
    --with-prelude none
}

generate word,reading,sense,label,sentence "$root/crates/dictionary/word/src/entity"
generate log "$root/crates/dictionary/log/src/entity"
