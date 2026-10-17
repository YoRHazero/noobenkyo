import { useMutation, useQueryClient } from '@tanstack/react-query'
import { studyKeys } from '@/features/study/study-keys'

/**
 * A mutation that ends the current question (rate, skip or report): on success
 * the zone status is refreshed and the page moves to the next question.
 */
export function useEndQuestion<T>(action: (value: T) => Promise<unknown>, onEnded: () => void) {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: action,
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: studyKeys.all })
      onEnded()
    },
  })
}
