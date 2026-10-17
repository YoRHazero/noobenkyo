import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { useEndQuestion } from '@/features/study/use-end-question'

export function useStartReviewGroup(onStarted: () => void) {
  return useEndQuestion(() => unwrap(commands.startReviewGroup()), onStarted)
}
