import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** `null` once the character is gone. */
export function useListeningCharacter(name: string) {
  return useQuery({
    queryKey: ['listening_character', name],
    queryFn: () => unwrap(commands.listeningCharacter(name)),
  })
}
