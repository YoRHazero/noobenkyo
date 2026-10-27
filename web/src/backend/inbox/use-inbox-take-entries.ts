import { commands, type InboxSource } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  source: InboxSource
  name: string
}

/** Resolves to the entry text; the entries leave the inbox. */
export function useInboxTakeEntries() {
  return useChange(({ source, name }: Input) => unwrap(commands.inboxTakeEntries(source, name)))
}
