import { ChevronDown, ChevronUp } from 'lucide-react'
import { cn } from '@/components/ui/cn'

type Props = {
  label: string
  count: number
  /** The count's color: orange for warnings, red for skipped. */
  tone: string
  open: boolean
  onToggle: () => void
}

/** A category of the summary, opening the list of its kinds. */
export function SummaryCategory({ label, count, tone, open, onToggle }: Props) {
  const Chevron = open ? ChevronUp : ChevronDown
  return (
    <button
      type="button"
      aria-expanded={open}
      className={cn(
        'flex items-center gap-1.5 rounded-md px-2 py-1 hover:bg-accent',
        open && 'bg-accent',
      )}
      onClick={onToggle}
    >
      <span className="text-muted-foreground">{label}</span>
      <span className={cn('font-semibold tabular-nums', tone)}>{count}</span>
      <Chevron className="size-3.5 text-muted-foreground" />
    </button>
  )
}
