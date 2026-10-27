import { commands, type ScriptTarget } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  name: string
  target: ScriptTarget
  text: string
}

/** Stores the script and removes it from the inbox; resolves to the episode's uuid. */
export function useInboxSaveScript() {
  return useChange(({ name, target, text }: Input) =>
    unwrap(commands.inboxSaveScript(name, target, text)),
  )
}
