/**
 * Moving episodes between the two boxes of the generation form. The chosen
 * ones keep the order the user gave them; ones added go to the end, in the
 * order of the world's episodes.
 */
export function addPicked(order: string[], chosen: string[], picked: ReadonlySet<string>) {
  return [...chosen, ...order.filter((uuid) => picked.has(uuid) && !chosen.includes(uuid))]
}

export function removePicked(chosen: string[], picked: ReadonlySet<string>) {
  return chosen.filter((uuid) => !picked.has(uuid))
}

/** `picked` with `uuid` picked if it was not, and not if it was. */
export function togglePicked(picked: ReadonlySet<string>, uuid: string) {
  const next = new Set(picked)
  if (next.has(uuid)) {
    next.delete(uuid)
  } else {
    next.add(uuid)
  }
  return next
}

/** The picks left after `moved` changed sides. */
export function withoutPicks(picked: ReadonlySet<string>, moved: readonly string[]) {
  return new Set([...picked].filter((uuid) => !moved.includes(uuid)))
}
