type Props = {
  label: string
  count: number
}

/** A plain count of the summary. */
export function SummaryFigure({ label, count }: Props) {
  return (
    <span className="flex items-center gap-1.5 px-2 py-1">
      <span className="text-muted-foreground">{label}</span>
      <span className="font-semibold tabular-nums">{count}</span>
    </span>
  )
}
