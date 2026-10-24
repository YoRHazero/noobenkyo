import type { ReactNode } from 'react'
import { cn } from '@/components/ui/cn'

type Props = {
  label: string
  /** Shown pressed: 自动 on, a subtitle shown, a panel open. */
  active?: boolean
  disabled?: boolean
  className?: string
  onClick: () => void
  children: ReactNode
}

/**
 * A button on the dark stage. It never takes the focus, so Space and Enter
 * always reach the theater's keys instead of pressing the last button. A
 * button that is on or off does not change under the mouse, so after a
 * click it plainly shows which one it is now.
 */
export function StageButton({ label, active, disabled, className, onClick, children }: Props) {
  const toggle = active !== undefined
  return (
    <button
      type="button"
      aria-label={label}
      aria-pressed={active}
      disabled={disabled}
      className={cn(
        'flex h-9 min-w-9 items-center justify-center gap-1 rounded-md px-2 text-sm text-white/45 transition-colors disabled:pointer-events-none disabled:opacity-30 [&_svg]:size-4',
        !toggle && 'hover:bg-white/10 hover:text-white/90',
        active && 'bg-white/15 text-white',
        className,
      )}
      onMouseDown={(event) => event.preventDefault()}
      onClick={onClick}
    >
      {children}
    </button>
  )
}
