import { useState, type ReactNode } from 'react'
import { Button } from '@/components/ui/button'

type Props = {
  /** Shown until the first click: a text, or an icon together with `ariaLabel`. */
  label: ReactNode
  ariaLabel?: string
  /** Shown after the first click; the second click acts. */
  confirmLabel: string
  /** The size before the first click; after it, the button fits its text. */
  size?: 'sm' | 'icon-sm'
  disabled?: boolean
  onConfirm: () => void
}

/** A button for something that cannot be undone here: it acts on the second click. */
export function ConfirmButton({
  label,
  ariaLabel,
  confirmLabel,
  size = 'sm',
  disabled,
  onConfirm,
}: Props) {
  const [armed, setArmed] = useState(false)
  return (
    <Button
      variant={armed ? 'destructive' : 'ghost'}
      size={armed ? 'sm' : size}
      aria-label={armed ? undefined : ariaLabel}
      disabled={disabled}
      onClick={() => (armed ? onConfirm() : setArmed(true))}
      onBlur={() => setArmed(false)}
    >
      {armed ? confirmLabel : label}
    </Button>
  )
}
