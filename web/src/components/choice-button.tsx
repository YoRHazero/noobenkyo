import type { ReactNode } from 'react'
import { Button } from '@/components/ui/button'
import { cn } from '@/components/ui/cn'

type Props = {
  chosen: boolean
  disabled?: boolean
  onClick: () => void
  children: ReactNode
}

/** One choice of a filter, shown pressed while chosen. */
export function ChoiceButton({ chosen, disabled, onClick, children }: Props) {
  return (
    <Button
      variant="ghost"
      size="sm"
      aria-pressed={chosen}
      disabled={disabled}
      className={cn('h-7 px-2.5', chosen && 'bg-accent text-foreground')}
      onClick={onClick}
    >
      {children}
    </Button>
  )
}
