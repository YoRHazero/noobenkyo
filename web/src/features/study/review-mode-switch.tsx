import type { ReviewMode } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { reviewModeLabel } from '@/features/study/labels'
import { useSetReviewMode } from '@/features/study/use-set-review-mode'

const modes: ReviewMode[] = ['Endless', 'FixedGroup']

type Props = {
  mode: ReviewMode
  onChanged: () => void
}

/** Switching settles every review sense and empties the zone. */
export function ReviewModeSwitch({ mode, onChanged }: Props) {
  const setMode = useSetReviewMode(onChanged)
  return (
    <div className="flex flex-col items-end gap-1">
      <div className="flex rounded-md border p-0.5">
        {modes.map((option) => (
          <Button
            key={option}
            size="sm"
            variant={option === mode ? 'secondary' : 'ghost'}
            disabled={setMode.isPending}
            onClick={() => option !== mode && setMode.mutate(option)}
          >
            {reviewModeLabel[option]}
          </Button>
        ))}
      </div>
      <ErrorText error={setMode.error} />
    </div>
  )
}
