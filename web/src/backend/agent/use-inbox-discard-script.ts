import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useInboxDiscardScript() {
  return useChange((name: string) => unwrap(commands.inboxDiscardScript(name)))
}
