/** Parses "1, 2, 4" into non-negative integers; `null` if any item is not one. */
export function parseNumberList(text: string): number[] | null {
  const items = text.split(/[\s,，、]+/).filter((item) => item !== '')
  const numbers = items.map(Number)
  return numbers.every((n) => Number.isInteger(n) && n >= 0) ? numbers : null
}
