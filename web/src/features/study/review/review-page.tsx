import { useNavigate } from '@tanstack/react-router'
import { useDictionaryStats } from '@/backend/dictionary/use-dictionary-stats'
import { useReviewStart } from '@/backend/review/use-review-start'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { ReviewEntrances } from '@/features/study/review/review-entrances'
import { sessionOptions, type ReviewMode } from '@/features/study/review/review-mode'

/**
 * The two ways into review. Starting one opens the session's own page,
 * which takes the whole window; the options come from the settings.
 */
export function ReviewPage() {
  const settings = useStudySettings()
  const stats = useDictionaryStats()
  const start = useReviewStart()
  const navigate = useNavigate()

  const begin = (mode: ReviewMode) => {
    if (!settings.data) {
      return
    }
    start.mutate(sessionOptions(settings.data.review.defaults, mode), {
      onSuccess: () => navigate({ to: '/review/session', search: { mode } }),
    })
  }

  return (
    <Page title="复习">
      <ReviewEntrances
        groupSize={settings.data?.review.defaults.size}
        toReview={stats.data?.stages.reduce((sum, stage) => sum + stage.senses, 0)}
        disabled={!settings.data || start.isPending}
        onStart={begin}
      />
      <ErrorText error={start.error ?? settings.error ?? stats.error} />
    </Page>
  )
}
