import type { Zone } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { useReportSentence } from '@/features/study/use-report-sentence'
import { useSkipSense } from '@/features/study/use-skip-sense'

type Props = {
  zone: Zone
  showsSentence: boolean
  onEnded: () => void
}

/** Leaving a question without rating: skip the sense, or report its sentence as wrong. */
export function QuestionActions({ zone, showsSentence, onEnded }: Props) {
  const skip = useSkipSense(zone, onEnded)
  const report = useReportSentence(zone, onEnded)
  const pending = skip.isPending || report.isPending
  return (
    <div className="flex flex-col items-end gap-1">
      <div className="flex gap-2">
        <Button variant="ghost" size="sm" disabled={pending} onClick={() => skip.mutate(undefined)}>
          跳过（已掌握）
        </Button>
        {showsSentence && (
          <Button
            variant="ghost"
            size="sm"
            disabled={pending}
            onClick={() => report.mutate(undefined)}
          >
            例句有误
          </Button>
        )}
      </div>
      <ErrorText error={skip.error ?? report.error} />
    </div>
  )
}
