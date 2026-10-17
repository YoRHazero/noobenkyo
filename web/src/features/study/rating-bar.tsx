import { useEffect } from 'react'
import type { Rating, Zone } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { cn } from '@/components/ui/cn'
import { ratingLabel } from '@/features/study/labels'
import { useRateAnswer } from '@/features/study/use-rate-answer'

const ratings: Rating[] = ['Known', 'Vague', 'Unknown']

const tone: Record<Rating, string> = {
  Known: 'border-known text-known',
  Vague: 'border-vague text-vague',
  Unknown: 'border-unknown text-unknown',
}

type Props = {
  zone: Zone
  prediction: Rating | null
  onEnded: () => void
}

/**
 * Self-rating. The prediction is preselected: Enter confirms it, 1/2/3 rate directly.
 */
export function RatingBar({ zone, prediction, onEnded }: Props) {
  const rate = useRateAnswer(zone, onEnded)
  const { mutate, isPending } = rate

  useEffect(() => {
    const keyDown = (event: KeyboardEvent) => {
      if (isPending || event.isComposing) {
        return
      }
      const index = ['1', '2', '3'].indexOf(event.key)
      const chosen = index >= 0 ? ratings[index] : event.key === 'Enter' ? prediction : null
      if (chosen) {
        event.preventDefault()
        mutate(chosen)
      }
    }
    window.addEventListener('keydown', keyDown)
    return () => window.removeEventListener('keydown', keyDown)
  }, [isPending, mutate, prediction])

  return (
    <div className="flex flex-col gap-2">
      <div className="flex gap-3">
        {ratings.map((rating, index) => (
          <Button
            key={rating}
            variant="outline"
            disabled={isPending}
            className={cn('min-w-24', prediction === rating && ['border-2', tone[rating]])}
            onClick={() => mutate(rating)}
          >
            {ratingLabel[rating]}
            <span className="text-xs text-muted-foreground">{index + 1}</span>
          </Button>
        ))}
      </div>
      <ErrorText error={rate.error} />
    </div>
  )
}
