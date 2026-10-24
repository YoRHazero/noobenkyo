import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useListeningDeleteEpisode() {
  return useChange((episode: string) => unwrap(commands.listeningDeleteEpisode(episode)))
}
