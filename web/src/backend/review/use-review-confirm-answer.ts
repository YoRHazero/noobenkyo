import { commands, type Rating } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useReviewConfirmAnswer() {
  return useChange((rating: Rating) => unwrap(commands.reviewConfirmAnswer(rating)))
}
