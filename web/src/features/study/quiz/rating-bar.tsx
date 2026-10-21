import { useEffect } from 'react'
import type { Rating } from '@/bindings'
import { Button } from '@/components/ui/button'
import { cn } from '@/components/ui/cn'
import { ratingLabel, ratings } from '@/features/study/quiz/labels'

const tone: Record<Rating, string> = {
  Known: 'border-known text-known',
  Vague: 'border-vague text-vague',
  Unknown: 'border-unknown text-unknown',
}

type Props = {
  /** Highlighted; Enter confirms it. */
  preselected: Rating | null
  disabled: boolean
  onConfirm: (rating: Rating) => void
}

/** Self-rating after the answer is revealed: Enter takes the preselection, 1/2/3 pick directly. */
export function RatingBar({ preselected, disabled, onConfirm }: Props) {
  useEffect(() => {
    const keyDown = (event: KeyboardEvent) => {
      if (disabled || event.isComposing) {
        return
      }
      const index = ['1', '2', '3'].indexOf(event.key)
      const chosen = index >= 0 ? ratings[index] : event.key === 'Enter' ? preselected : null
      if (chosen) {
        event.preventDefault()
        onConfirm(chosen)
      }
    }
    window.addEventListener('keydown', keyDown)
    return () => window.removeEventListener('keydown', keyDown)
  }, [disabled, preselected, onConfirm])

  return (
    <div className="flex gap-3">
      {ratings.map((rating, index) => (
        <Button
          key={rating}
          variant="outline"
          disabled={disabled}
          className={cn('min-w-24', preselected === rating && ['border-2', tone[rating]])}
          onClick={() => onConfirm(rating)}
        >
          {ratingLabel[rating]}
          <span className="text-xs text-muted-foreground">{index + 1}</span>
        </Button>
      ))}
    </div>
  )
}
