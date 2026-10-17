import type { ZoneStatus } from '@/bindings'
import { reviewModeLabel } from '@/features/study/labels'

type Props = {
  status: ZoneStatus
}

/** Counts of a zone: senses in it, and today's new senses or the review mode. */
export function ZoneSummary({ status }: Props) {
  return (
    <div className="flex gap-6 text-sm text-muted-foreground">
      <span>
        区内 {status.items} / {status.capacity}
      </span>
      {status.zone === 'Learning' ? (
        <span>
          今日新词 {status.new_today} / {status.daily_new_limit}
        </span>
      ) : (
        <span>{reviewModeLabel[status.review_mode]}</span>
      )}
    </div>
  )
}
