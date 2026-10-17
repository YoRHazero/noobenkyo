import { useId, useState } from 'react'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { parseNumberList } from '@/features/settings/parse-number-list'

type Props = {
  label: string
  value: number[]
  onChange: (value: number[]) => void
}

/** A list of whole numbers typed as "1, 2, 4". */
export function NumberListField({ label, value, onChange }: Props) {
  const id = useId()
  const [text, setText] = useState(value.join(', '))
  const parsed = parseNumberList(text)
  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      <Input
        id={id}
        value={text}
        aria-invalid={parsed === null}
        onChange={(event) => {
          setText(event.target.value)
          const numbers = parseNumberList(event.target.value)
          if (numbers !== null) {
            onChange(numbers)
          }
        }}
      />
    </div>
  )
}
