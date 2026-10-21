import { useEffect, useState } from 'react'
import { useDictionaryStats } from '@/backend/dictionary/use-dictionary-stats'
import { useReviewEnd } from '@/backend/review/use-review-end'
import { useReviewStart } from '@/backend/review/use-review-start'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { ReviewEntrances } from '@/features/study/review/review-entrances'
import { sessionOptions, type ReviewMode } from '@/features/study/review/review-mode'
import { ReviewSession } from '@/features/study/review/review-session'

type Phase =
  | { kind: 'entrances' }
  /** `run` counts sessions, so another group starts with fresh counts. */
  | { kind: 'session'; mode: ReviewMode; run: number }

/**
 * Review: the two entrances, then one session at a time. The options come
 * from the settings; leaving the page ends the session.
 */
export function ReviewPage() {
  const settings = useStudySettings()
  const stats = useDictionaryStats()
  const start = useReviewStart()
  const end = useReviewEnd()
  const [phase, setPhase] = useState<Phase>({ kind: 'entrances' })

  const endSession = end.mutate
  useEffect(() => () => endSession(), [endSession])

  const begin = (mode: ReviewMode) => {
    if (!settings.data) {
      return
    }
    start.mutate(sessionOptions(settings.data.review.defaults, mode), {
      onSuccess: () =>
        setPhase((last) => ({
          kind: 'session',
          mode,
          run: last.kind === 'session' ? last.run + 1 : 0,
        })),
    })
  }
  const back = () => {
    setPhase({ kind: 'entrances' })
    end.mutate()
  }

  return (
    <Page title="复习">
      {phase.kind === 'entrances' ? (
        <ReviewEntrances
          groupSize={settings.data?.review.defaults.size}
          toReview={stats.data?.stages.reduce((sum, stage) => sum + stage.senses, 0)}
          disabled={!settings.data || start.isPending}
          onStart={begin}
        />
      ) : (
        <ReviewSession
          key={phase.run}
          mode={phase.mode}
          starting={start.isPending}
          onAgain={() => begin(phase.mode)}
          onBack={back}
        />
      )}
      <ErrorText error={start.error ?? settings.error ?? stats.error} />
    </Page>
  )
}
