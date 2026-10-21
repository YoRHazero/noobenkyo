import { useQueryClient } from '@tanstack/react-query'
import { commands, type SessionOptions } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

/**
 * Starts a session. The last session's quiz, or its "nothing left", is
 * dropped first, so the new session never shows it while its first quiz loads.
 */
export function useReviewStart() {
  const queryClient = useQueryClient()
  return useChange((options: SessionOptions) => {
    queryClient.removeQueries({ queryKey: ['review_next'] })
    return unwrap(commands.reviewStart(options))
  })
}
