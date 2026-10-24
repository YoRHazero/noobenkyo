import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** `null` once the episode is gone. */
export function useListeningEpisode(episode: string) {
  return useQuery({
    queryKey: ['listening_episode', episode],
    queryFn: () => unwrap(commands.listeningEpisode(episode)),
  })
}
