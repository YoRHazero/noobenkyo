import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useReviewReportSentence() {
  return useChange(() => unwrap(commands.reviewReportSentence()))
}
