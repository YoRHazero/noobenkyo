import { useId } from 'react'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

type Props = {
  label: string
  value: number
  onChange: (value: number) => void
}

/** A whole-number setting. */
export function NumberField({ label, value, onChange }: Props) {
  const id = useId()
  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      <Input
        id={id}
        type="number"
        min={0}
        step={1}
        value={value}
        onChange={(event) => onChange(Math.max(0, Math.trunc(Number(event.target.value))))}
      />
    </div>
  )
}
