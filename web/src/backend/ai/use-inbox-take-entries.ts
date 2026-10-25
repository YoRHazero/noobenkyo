import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

/** Resolves to the entry text; the entries leave the inbox. */
export function useInboxTakeEntries() {
  return useChange((name: string) => unwrap(commands.inboxTakeEntries(name)))
}
