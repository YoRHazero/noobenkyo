import { commands, type Rating } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useLearnConfirmAnswer() {
  return useChange((rating: Rating) => unwrap(commands.learnConfirmAnswer(rating)))
}
