/** Romaji still being typed, which WanaKana has not turned into kana yet. */
const pendingRomaji = /[a-zA-Z']/

/** Number of kana typed so far; pending romaji does not count. */
export function countKana(value: string): number {
  return [...value].filter((c) => !pendingRomaji.test(c)).length
}

/** Whether the value stays within the kana limit (`null` means no limit). */
export function fitsKanaLimit(value: string, limit: number | null): boolean {
  return limit === null || countKana(value) <= limit
}
