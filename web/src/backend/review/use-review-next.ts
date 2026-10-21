import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Asking again before answering gives the same quiz. */
export function useReviewNext() {
  return useQuery({
    queryKey: ['review_next'],
    queryFn: () => unwrap(commands.reviewNext()),
  })
}
