import type { TextRange } from '@/features/dictionary/import/line-range'

/**
 * Where `query` appears in the text, taken literally and ignoring case, in
 * text order and not overlapping. Nothing for an empty query.
 */
export function findMatches(text: string, query: string): TextRange[] {
  if (query === '') {
    return []
  }
  const pattern = new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'giu')
  return Array.from(text.matchAll(pattern), (match) => ({
    start: match.index,
    end: match.index + match[0].length,
  }))
}
