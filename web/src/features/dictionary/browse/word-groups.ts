import type { SearchRow, WordKey } from '@/bindings'

export type WordGroup = {
  word: WordKey
  /** In the order search gives them. */
  rows: SearchRow[]
}

/**
 * A page of rows grouped by word. Search puts a word's rows next to each
 * other, so a group is a run of rows with the same word; a word cut by the
 * page's edge starts the next page again.
 */
export function wordGroups(rows: SearchRow[]): WordGroup[] {
  const groups: WordGroup[] = []
  for (const row of rows) {
    const last = groups.at(-1)
    if (last && sameWord(last.word, row.word)) {
      last.rows.push(row)
    } else {
      groups.push({ word: row.word, rows: [row] })
    }
  }
  return groups
}

function sameWord(a: WordKey, b: WordKey): boolean {
  return a.writing === b.writing && a.origin === b.origin
}
