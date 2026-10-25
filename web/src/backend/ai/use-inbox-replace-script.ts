import { commands, type ScriptTarget } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  name: string
  target: ScriptTarget
  text: string
}

/** Resolves to the problems that kept the edit out; empty once it is written back. */
export function useInboxReplaceScript() {
  return useChange(({ name, target, text }: Input) =>
    unwrap(commands.inboxReplaceScript(name, target, text)),
  )
}
