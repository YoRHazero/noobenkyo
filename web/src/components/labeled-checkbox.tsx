import { useId } from 'react'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'

type Props = {
  label: string
  checked: boolean
  onChange: (checked: boolean) => void
}

export function LabeledCheckbox({ label, checked, onChange }: Props) {
  const id = useId()
  return (
    <div className="flex items-center gap-2">
      <Checkbox id={id} checked={checked} onCheckedChange={(value) => onChange(value === true)} />
      <Label htmlFor={id}>{label}</Label>
    </div>
  )
}
