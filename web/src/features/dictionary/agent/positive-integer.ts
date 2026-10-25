/** The most a count may be: the backend takes it as a 32-bit unsigned integer. */
const largest = 4294967295

/** A count typed into a form: digits only, at least 1; `null` for anything else. */
export function parsePositiveInteger(text: string): number | null {
  const trimmed = text.trim()
  if (!/^\d+$/.test(trimmed)) {
    return null
  }
  const value = Number(trimmed)
  return value >= 1 && value <= largest ? value : null
}
