import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  episode: string
  text: string
}

/** Continues the episode with the script's lines and summary. */
export function useListeningAppendScript() {
  return useChange(({ episode, text }: Input) =>
    unwrap(commands.listeningAppendScript(episode, text)),
  )
}
