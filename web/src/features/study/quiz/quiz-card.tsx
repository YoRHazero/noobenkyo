import type { QuizPayload, Rating, RevealPayload } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { AnswerForm } from '@/features/study/quiz/answer-form'
import { ConfirmButton } from '@/components/confirm-button'
import { questionTypeLabel } from '@/features/study/quiz/labels'
import { preselectedRating } from '@/features/study/quiz/preselected-rating'
import { QuizFace } from '@/features/study/quiz/quiz-face'
import { RatingBar } from '@/features/study/quiz/rating-bar'
import { RevealPanel } from '@/features/study/quiz/reveal-panel'
import { StepDots } from '@/features/study/quiz/step-dots'

type Props = {
  quiz: QuizPayload
  /** Present once the answer is submitted. */
  reveal: RevealPayload | undefined
  /** A command of this quiz is running. */
  busy: boolean
  error: Error | null
  onSubmit: (input: string) => void
  onConfirm: (rating: Rating) => void
  onSkip: () => void
  onReport: () => void
}

/**
 * One quiz, the same on both lines: the question and its answer box, then
 * the revealed answer and the self-rating. Skipping is always possible;
 * reporting the sentence only while the user can see it.
 */
export function QuizCard({ quiz, reveal, busy, error, ...on }: Props) {
  const { view } = quiz
  const showsSentence = reveal !== undefined || quiz.sentence !== null
  return (
    <div className="flex flex-col gap-6 rounded-xl border bg-card p-6 shadow-sm">
      <div className="flex items-center justify-between">
        <span className="font-medium">{questionTypeLabel[view.question_type]}</span>
        <StepDots remaining={view.remaining} steps={view.steps} />
      </div>
      {reveal ? (
        <>
          <RevealPanel reveal={reveal} />
          <RatingBar
            preselected={preselectedRating(reveal.view.judgement)}
            disabled={busy}
            onConfirm={on.onConfirm}
          />
        </>
      ) : (
        <>
          <QuizFace quiz={quiz} />
          <AnswerForm answer={view.answer} disabled={busy} onSubmit={on.onSubmit} />
        </>
      )}
      <div className="flex items-center justify-between gap-4">
        <ErrorText error={error} />
        <div className="ml-auto flex gap-2">
          <ConfirmButton
            label="跳过（已掌握）"
            confirmLabel="确认跳过"
            disabled={busy}
            onConfirm={on.onSkip}
          />
          {showsSentence && (
            <ConfirmButton
              label="例句有错"
              confirmLabel="确认删除"
              disabled={busy}
              onConfirm={on.onReport}
            />
          )}
        </div>
      </div>
    </div>
  )
}
