import { commands, type InboxSource, type ScriptTarget } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  source: InboxSource
  name: string
  target: ScriptTarget
  text: string
}

/** Stores the script and removes it from the inbox; resolves to the episode's uuid. */
export function useInboxSaveScript() {
  return useChange(({ source, name, target, text }: Input) =>
    unwrap(commands.inboxSaveScript(source, name, target, text)),
  )
}
