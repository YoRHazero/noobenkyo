import { useId } from 'react'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { parsePositiveInteger } from '@/features/dictionary/agent/positive-integer'

type Props = {
  label: string
  /** As typed; marked red unless it is a whole number from 1. */
  value: string
  onChange: (value: string) => void
}

/** A count for an agent's task: how many rows, words or sentences. */
export function CountField({ label, value, onChange }: Props) {
  const id = useId()
  return (
    <div className="flex items-center gap-3">
      <Label htmlFor={id} className="w-24">
        {label}
      </Label>
      <Input
        id={id}
        inputMode="numeric"
        value={value}
        aria-invalid={parsePositiveInteger(value) === null}
        className="h-8 w-24"
        onChange={(event) => onChange(event.target.value)}
      />
    </div>
  )
}
