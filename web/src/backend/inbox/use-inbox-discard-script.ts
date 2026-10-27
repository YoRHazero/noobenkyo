import { commands, type InboxSource } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  source: InboxSource
  name: string
}

export function useInboxDiscardScript() {
  return useChange(({ source, name }: Input) => unwrap(commands.inboxDiscardScript(source, name)))
}
