import type { QuizPayload } from '@/bindings'
import { useLearnConfirmAnswer } from '@/backend/learn/use-learn-confirm-answer'
import { useLearnReportSentence } from '@/backend/learn/use-learn-report-sentence'
import { useLearnSkip } from '@/backend/learn/use-learn-skip'
import { useLearnSubmitAnswer } from '@/backend/learn/use-learn-submit-answer'
import { QuizCard } from '@/features/study/quiz/quiz-card'

type Props = {
  quiz: QuizPayload
}

/** One quiz on the learning line: its commands, and what they have returned. */
export function LearnQuiz({ quiz }: Props) {
  const submit = useLearnSubmitAnswer()
  const confirm = useLearnConfirmAnswer()
  const skip = useLearnSkip()
  const report = useLearnReportSentence()
  return (
    <QuizCard
      quiz={quiz}
      reveal={submit.data}
      busy={submit.isPending || confirm.isPending || skip.isPending || report.isPending}
      error={submit.error ?? confirm.error ?? skip.error ?? report.error}
      onSubmit={submit.mutate}
      onConfirm={confirm.mutate}
      onSkip={() => skip.mutate()}
      onReport={() => report.mutate()}
    />
  )
}
