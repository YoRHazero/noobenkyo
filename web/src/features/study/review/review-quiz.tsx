import type { QuizPayload } from '@/bindings'
import { useReviewConfirmAnswer } from '@/backend/review/use-review-confirm-answer'
import { useReviewReportSentence } from '@/backend/review/use-review-report-sentence'
import { useReviewSkip } from '@/backend/review/use-review-skip'
import { useReviewSubmitAnswer } from '@/backend/review/use-review-submit-answer'
import { QuizCard } from '@/features/study/quiz/quiz-card'

type Props = {
  quiz: QuizPayload
  onAnswered: () => void
  onSkipped: () => void
}

/**
 * One quiz of a review session. The counts are told once the change is
 * done; by then the next quiz may have replaced this one, so they are not
 * left to callbacks of an unmounted card.
 */
export function ReviewQuiz({ quiz, onAnswered, onSkipped }: Props) {
  const submit = useReviewSubmitAnswer()
  const confirm = useReviewConfirmAnswer()
  const skip = useReviewSkip()
  const report = useReviewReportSentence()
  return (
    <QuizCard
      quiz={quiz}
      reveal={submit.data}
      busy={submit.isPending || confirm.isPending || skip.isPending || report.isPending}
      error={submit.error ?? confirm.error ?? skip.error ?? report.error}
      onSubmit={submit.mutate}
      onConfirm={(rating) => void confirm.mutateAsync(rating).then(onAnswered, () => {})}
      onSkip={() => void skip.mutateAsync().then(onSkipped, () => {})}
      onReport={() => report.mutate()}
    />
  )
}
