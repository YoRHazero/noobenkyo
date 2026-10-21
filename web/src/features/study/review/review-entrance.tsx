import type { ReactNode, Ref } from 'react'
import { cn } from '@/components/ui/cn'

type Props = {
  ref?: Ref<HTMLButtonElement>
  title: ReactNode
  label: string
  /** `null` for no limit; `undefined` while it loads. */
  count: number | null | undefined
  disabled: boolean
  /** Set as `data-active`, for a card styled by its own effect. */
  active?: boolean
  className?: string
  /** Drawn inside the card, after its text. */
  children?: ReactNode
  onClick: () => void
  onMouseEnter?: () => void
  onMouseLeave?: () => void
}

/**
 * One way into review: a card that starts its session. Its text takes the
 * card's color, dimmed where it is secondary, so a card can change color.
 */
export function ReviewEntrance({
  ref,
  title,
  label,
  count,
  disabled,
  active,
  className,
  children,
  ...on
}: Props) {
  return (
    <button
      ref={ref}
      type="button"
      disabled={disabled}
      data-active={active}
      className={cn(
        'relative flex flex-col gap-3 rounded-xl border bg-card p-6 text-left shadow-sm transition-shadow hover:shadow-md disabled:cursor-wait',
        className,
      )}
      {...on}
    >
      <span className="text-xl font-semibold">{title}</span>
      <span className="flex items-baseline gap-1 text-sm">
        <span className="opacity-60">{label}：</span>
        <span className="text-2xl font-semibold tabular-nums">
          {count === undefined ? '—' : (count ?? '不限')}
        </span>
        {typeof count === 'number' && <span className="opacity-60">个</span>}
      </span>
      {children}
    </button>
  )
}
