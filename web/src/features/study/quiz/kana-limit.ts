/** Romaji still being typed, which WanaKana has not turned into kana yet. */
const pendingRomaji = /[a-zA-Z']/

/** Kana typed so far; pending romaji does not count. */
export function countKana(value: string): number {
  return [...value].filter((c) => !pendingRomaji.test(c)).length
}

/**
 * Whether the value stays within the limit (`null`: no limit). Pending
 * romaji will become at least one more kana, so it takes a place too.
 */
export function fitsKanaLimit(value: string, limit: number | null): boolean {
  const pending = [...value].some((c) => pendingRomaji.test(c)) ? 1 : 0
  return limit === null || countKana(value) + pending <= limit
}
