import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import { useReviewStart } from '@/backend/review/use-review-start'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { ErrorText } from '@/components/error-text'
import { sessionOptions, type ReviewMode } from '@/features/study/review/review-mode'
import { ReviewSession } from '@/features/study/review/review-session'

type Props = {
  mode: ReviewMode
}

/**
 * A session started on the review page, over the whole window. Leaving this
 * page ends the session; the router does that, not this component, so a
 * development remount does not end a session that just began.
 */
export function ReviewSessionPage({ mode }: Props) {
  const settings = useStudySettings()
  const start = useReviewStart()
  const navigate = useNavigate()
  /** Counts sessions, so another group starts with fresh counts. */
  const [run, setRun] = useState(0)

  const again = () => {
    if (settings.data) {
      start.mutate(sessionOptions(settings.data.review.defaults, mode), {
        onSuccess: () => setRun((last) => last + 1),
      })
    }
  }

  return (
    <div className="flex flex-col gap-4">
      <ReviewSession
        key={run}
        mode={mode}
        starting={start.isPending}
        onAgain={again}
        onBack={() => navigate({ to: '/review' })}
      />
      <ErrorText error={start.error ?? settings.error} />
    </div>
  )
}
