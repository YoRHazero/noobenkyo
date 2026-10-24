import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  episode: string
  number: number
  title: string
  summary: string
}

/** Resolves to the problems that rejected the three, empty when they were replaced. */
export function useListeningUpdateEpisode() {
  return useChange(({ episode, number, title, summary }: Input) =>
    unwrap(commands.listeningUpdateEpisode(episode, number, title, summary)),
  )
}
