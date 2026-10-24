/**
 * What to select once `removed` is gone from `names`: the one after it, or
 * the one before when it was last; `null` when nothing is left.
 */
export function nextSelection(names: readonly string[], removed: string): string | null {
  const index = names.indexOf(removed)
  const rest = names.filter((name) => name !== removed)
  if (rest.length === 0) {
    return null
  }
  return rest[Math.min(Math.max(index, 0), rest.length - 1)] ?? null
}
