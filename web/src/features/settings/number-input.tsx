import { Input } from '@/components/ui/input'
import { parseCount } from '@/features/settings/number-fields'

type Props = {
  label: string
  value: string
  /** Written after the box: "个", "天/档" … */
  unit?: string
  onChange: (value: string) => void
}

/** A whole number as typed; anything else is marked until it is fixed. */
export function NumberInput({ label, value, unit, onChange }: Props) {
  const invalid = parseCount(value) === null
  return (
    <span className="flex items-center gap-1.5 text-sm">
      <Input
        value={value}
        inputMode="numeric"
        aria-label={label}
        aria-invalid={invalid}
        className="h-8 w-20 tabular-nums"
        onChange={(event) => onChange(event.target.value)}
      />
      {unit && <span className="text-muted-foreground">{unit}</span>}
    </span>
  )
}
