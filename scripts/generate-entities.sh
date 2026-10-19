#!/usr/bin/env bash
# Regenerates the SeaORM entities from the migrations.
# Migrations are the only source of the schema; entities are never edited by hand.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

db="$tmp/dictionary.db"  # created by the migrate example in $tmp
cargo run --quiet --manifest-path "$root/Cargo.toml" -p dictionary-word --example migrate -- "$tmp"

out="$root/crates/dictionary/word/src/entity"
rm -rf "$out"
sea-orm-cli generate entity \
  --database-url "sqlite://$db" \
  --tables word,reading,sense,label,sentence \
  --output-dir "$out" \
  --with-prelude none
