import { commands, type LineId } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

/** Resolves to how many lines were deleted. */
export function useListeningDeleteLines() {
  return useChange((lines: LineId[]) => unwrap(commands.listeningDeleteLines(lines)))
}
