import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

export function useListeningCharacters() {
  return useQuery({
    queryKey: ['listening_characters'],
    queryFn: () => unwrap(commands.listeningCharacters()),
  })
}
