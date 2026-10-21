import { X } from 'lucide-react'

type Props = {
  /** "活用形", "警告" … */
  label: string
  onClear: () => void
}

/** Which rows the table is narrowed to; clearing it shows every row. */
export function FilterChip({ label, onClear }: Props) {
  return (
    <span className="flex items-center gap-1 rounded-md bg-accent py-1 pr-1 pl-2 text-sm">
      只看：{label}
      <button
        type="button"
        aria-label="显示全部"
        className="rounded-sm p-0.5 text-muted-foreground hover:bg-background hover:text-foreground"
        onClick={onClear}
      >
        <X className="size-3.5" />
      </button>
    </span>
  )
}
