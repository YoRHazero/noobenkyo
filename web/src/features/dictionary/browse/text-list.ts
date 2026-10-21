/**
 * "明天；明日" → ["明天", "明日"]: split at full- or half-width semicolons,
 * each item trimmed, blank ones dropped.
 */
export function splitList(text: string): string[] {
  return text
    .split(/[；;]/)
    .map((item) => item.trim())
    .filter((item) => item !== '')
}

/** ["明天", "明日"] → "明天；明日", as lists are shown. */
export function joinList(items: string[]): string {
  return items.join('；')
}
