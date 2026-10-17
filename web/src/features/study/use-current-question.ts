import { useQuery } from '@tanstack/react-query'
import { commands, type Zone } from '@/bindings'
import { unwrap } from '@/components/result'
import { studyKeys } from '@/features/study/study-keys'

/** The zone's current question; `round` moves on after each answered question. */
export function useCurrentQuestion(zone: Zone, round: number) {
  return useQuery({
    queryKey: studyKeys.question(zone, round),
    queryFn: () => unwrap(commands.currentQuestion(zone)),
    staleTime: Infinity,
    gcTime: 0,
  })
}
