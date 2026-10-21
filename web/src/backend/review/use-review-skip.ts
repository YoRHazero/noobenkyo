import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useReviewSkip() {
  return useChange(() => unwrap(commands.reviewSkip()))
}
