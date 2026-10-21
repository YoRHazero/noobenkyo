import { cn } from '@/components/ui/cn'
import type { RowStatus } from '@/features/dictionary/import/preview/row-status'

type Props = {
  status: RowStatus | null
}

/** The row's status, orange for a warning and red when skipped. */
export function StatusCell({ status }: Props) {
  if (!status) {
    return <span />
  }
  return (
    <span
      className={cn('truncate', status.kind === 'warning' ? 'text-warning' : 'text-destructive')}
    >
      {status.text}
    </span>
  )
}
