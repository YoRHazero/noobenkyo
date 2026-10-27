import { commands, type InboxSource, type ScriptTarget } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  source: InboxSource
  name: string
  target: ScriptTarget
  text: string
}

/** Resolves to the problems that kept the edit out; empty once it is written back. */
export function useInboxReplaceScript() {
  return useChange(({ source, name, target, text }: Input) =>
    unwrap(commands.inboxReplaceScript(source, name, target, text)),
  )
}
