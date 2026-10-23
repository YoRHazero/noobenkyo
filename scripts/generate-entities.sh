#!/usr/bin/env bash
# Regenerates the SeaORM entities from the migrations.
# Migrations are the only source of the schema; entities are never edited by hand.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# generate <package> <database file> <tables> <entity directory>
generate() {
  local package="$1" database="$2" tables="$3" out="$root/$4"
  # The package's migrate example creates the database in $tmp.
  cargo run --quiet --manifest-path "$root/Cargo.toml" -p "$package" --example migrate -- "$tmp"
  rm -rf "$out"
  sea-orm-cli generate entity \
    --database-url "sqlite://$tmp/$database" \
    --tables "$tables" \
    --output-dir "$out" \
    --with-prelude none
}

generate dictionary-word dictionary.db word,reading,sense,label,sentence \
  crates/dictionary/word/src/entity
generate listening-drama listening.db character,reference,world,cast,episode,line,line_audio \
  crates/listening/drama/src/entity
