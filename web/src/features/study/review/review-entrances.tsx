import { EndlessEntrance } from '@/features/study/review/endless-entrance'
import { ReviewEntrance } from '@/features/study/review/review-entrance'
import type { ReviewMode } from '@/features/study/review/review-mode'

type Props = {
  /** From the settings; `undefined` while they load. */
  groupSize: number | null | undefined
  /** As the home page counts it; `undefined` while it loads. */
  toReview: number | undefined
  disabled: boolean
  onStart: (mode: ReviewMode) => void
}

/** The two ways into review, open whatever the counts say, as on the home page. */
export function ReviewEntrances({ groupSize, toReview, disabled, onStart }: Props) {
  return (
    <div className="grid grid-cols-2 gap-4">
      <ReviewEntrance
        title="分组复习"
        label="每组"
        count={groupSize}
        disabled={disabled}
        onClick={() => onStart('group')}
      />
      <EndlessEntrance toReview={toReview} disabled={disabled} onClick={() => onStart('endless')} />
    </div>
  )
}
