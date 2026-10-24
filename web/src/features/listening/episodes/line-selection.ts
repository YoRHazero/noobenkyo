import type { LineId } from '@/bindings'

/** The lines ticked for deleting, and the one a Shift-click extends from. */
export type LineSelection = {
  ids: ReadonlySet<LineId>
  anchor: LineId | null
}

export const noSelection: LineSelection = { ids: new Set(), anchor: null }

/**
 * A click on a line's box. A plain click ticks or unticks that line and makes
 * it the anchor; a Shift-click ticks every line from the anchor to it.
 */
export function clickLine(
  selection: LineSelection,
  order: readonly LineId[],
  id: LineId,
  shift: boolean,
): LineSelection {
  const from = selection.anchor === null ? -1 : order.indexOf(selection.anchor)
  const to = order.indexOf(id)
  if (shift && from !== -1 && to !== -1) {
    const range = order.slice(Math.min(from, to), Math.max(from, to) + 1)
    return { ids: new Set([...selection.ids, ...range]), anchor: selection.anchor }
  }
  const ids = new Set(selection.ids)
  if (!ids.delete(id)) {
    ids.add(id)
  }
  return { ids, anchor: id }
}
