import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** The worlds whose cast has the character. */
export function useListeningCharacterUsage(name: string) {
  return useQuery({
    queryKey: ['listening_character_usage', name],
    queryFn: () => unwrap(commands.listeningCharacterUsage(name)),
  })
}
