import { useState } from 'react'
import { useReviewNext } from '@/backend/review/use-review-next'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ReviewDone } from '@/features/study/review/review-done'
import { reviewModeLabel, type ReviewMode } from '@/features/study/review/review-mode'
import { ReviewQuiz } from '@/features/study/review/review-quiz'

type Props = {
  mode: ReviewMode
  /** Another group is being started. */
  starting: boolean
  onAgain: () => void
  onBack: () => void
}

/**
 * One session: quiz after quiz until the pool and the queue are empty. The
 * backend does not say how many are left, so the page counts what was done.
 */
export function ReviewSession({ mode, starting, onAgain, onBack }: Props) {
  const next = useReviewNext()
  /** Confirmed answers. */
  const [answered, setAnswered] = useState(0)
  /** Confirmed or skipped: whether the session had anything to review. */
  const [handled, setHandled] = useState(0)

  if (next.data === null) {
    return (
      <ReviewDone
        mode={mode}
        answered={answered}
        hadAny={handled > 0}
        starting={starting}
        onAgain={onAgain}
        onBack={onBack}
      />
    )
  }
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between text-sm">
        <span className="font-medium">{reviewModeLabel[mode]}</span>
        <div className="flex items-center gap-3">
          <span className="text-muted-foreground tabular-nums">已答：{answered}</span>
          <Button variant="outline" size="sm" onClick={onBack}>
            结束
          </Button>
        </div>
      </div>
      <ErrorText error={next.error} />
      {next.isPending && <p className="text-sm text-muted-foreground">加载中…</p>}
      {next.data && (
        <ReviewQuiz
          key={next.dataUpdatedAt}
          quiz={next.data}
          onAnswered={() => {
            setAnswered((n) => n + 1)
            setHandled((n) => n + 1)
          }}
          onSkipped={() => setHandled((n) => n + 1)}
        />
      )}
    </div>
  )
}
