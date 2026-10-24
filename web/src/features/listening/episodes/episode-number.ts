/** An episode number as typed: any finite number, fractions included; `null` otherwise. */
export function parseEpisodeNumber(text: string): number | null {
  const trimmed = text.trim()
  if (trimmed === '') {
    return null
  }
  const number = Number(trimmed)
  return Number.isFinite(number) ? number : null
}
