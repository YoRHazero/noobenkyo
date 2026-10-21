import { useMutation } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Submitting changes no read, so nothing is invalidated. */
export function useReviewSubmitAnswer() {
  return useMutation({
    mutationFn: (input: string) => unwrap(commands.reviewSubmitAnswer(input)),
  })
}
