import { commands, type Rating, type Zone } from '@/bindings'
import { unwrap } from '@/components/result'
import { useEndQuestion } from '@/features/study/use-end-question'

export function useRateAnswer(zone: Zone, onEnded: () => void) {
  return useEndQuestion((rating: Rating) => unwrap(commands.rateAnswer(zone, rating)), onEnded)
}
