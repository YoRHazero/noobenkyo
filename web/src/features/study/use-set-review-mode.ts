import { commands, type ReviewMode } from '@/bindings'
import { unwrap } from '@/components/result'
import { useEndQuestion } from '@/features/study/use-end-question'

/** Switching the mode empties the review zone, so it also ends the current question. */
export function useSetReviewMode(onChanged: () => void) {
  return useEndQuestion((mode: ReviewMode) => unwrap(commands.setReviewMode(mode)), onChanged)
}
