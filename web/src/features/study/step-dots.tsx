import { cn } from '@/components/ui/cn'

type Props = {
  remaining: number
  steps: number
}

/** How far the sense is from full: filled dots are steps done. */
export function StepDots({ remaining, steps }: Props) {
  return (
    <div className="flex gap-1.5" title={`还差 ${remaining} 步`}>
      {Array.from({ length: steps }, (_, index) => (
        <span
          key={index}
          className={cn(
            'size-2.5 rounded-full',
            index < steps - remaining ? 'bg-known' : 'bg-muted',
          )}
        />
      ))}
    </div>
  )
}
