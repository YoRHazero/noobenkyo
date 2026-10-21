import type { SearchRow, SenseKey } from '@/bindings'

/** Tells rows apart: their natural key, down to the layer they reach. */
export function rowKey(row: SearchRow): string {
  return JSON.stringify([
    row.word.writing,
    row.word.origin,
    row.kana,
    row.sense?.primary_gloss ?? null,
  ])
}

/** The key of the row a sense is shown on. */
export function senseRowKey(key: SenseKey): string {
  return JSON.stringify([key.writing, key.origin, key.kana, key.primary_gloss])
}
