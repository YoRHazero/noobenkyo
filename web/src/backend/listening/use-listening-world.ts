import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** `null` once the world is gone. */
export function useListeningWorld(name: string) {
  return useQuery({
    queryKey: ['listening_world', name],
    queryFn: () => unwrap(commands.listeningWorld(name)),
  })
}
