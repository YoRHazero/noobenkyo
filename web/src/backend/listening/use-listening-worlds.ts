import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

export function useListeningWorlds() {
  return useQuery({
    queryKey: ['listening_worlds'],
    queryFn: () => unwrap(commands.listeningWorlds()),
  })
}
