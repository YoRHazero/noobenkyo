import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** The world's episodes in their order: by number, then as they were made. */
export function useListeningEpisodes(world: string) {
  return useQuery({
    queryKey: ['listening_episodes', world],
    queryFn: () => unwrap(commands.listeningEpisodes(world)),
  })
}
