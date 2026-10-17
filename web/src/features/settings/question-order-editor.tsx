import { ArrowDown, ArrowUp, Plus, X } from 'lucide-react'
import type { QuestionType } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { questionTypeLabel } from '@/features/settings/labels'

const allTypes: QuestionType[] = ['Sentence', 'Translation', 'Reading', 'Comprehensive']

type Props = {
  label: string
  value: QuestionType[]
  onChange: (value: QuestionType[]) => void
}

/** A question order: reorder, remove and add question types. */
export function QuestionOrderEditor({ label, value, onChange }: Props) {
  const move = (index: number, offset: number) => {
    const next = [...value]
    const [item] = next.splice(index, 1)
    if (item) {
      next.splice(index + offset, 0, item)
      onChange(next)
    }
  }
  const missing = allTypes.filter((type) => !value.includes(type))

  return (
    <div className="flex flex-col gap-1.5">
      <Label>{label}</Label>
      <div className="flex flex-wrap items-center gap-2">
        {value.map((type, index) => (
          <span
            key={type}
            className="flex items-center gap-0.5 rounded-md border py-0.5 pr-0.5 pl-2 text-sm"
          >
            {index + 1}. {questionTypeLabel[type]}
            <Button
              variant="ghost"
              size="icon"
              className="size-6"
              disabled={index === 0}
              onClick={() => move(index, -1)}
            >
              <ArrowUp />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="size-6"
              disabled={index === value.length - 1}
              onClick={() => move(index, 1)}
            >
              <ArrowDown />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="size-6"
              onClick={() => onChange(value.filter((t) => t !== type))}
            >
              <X />
            </Button>
          </span>
        ))}
        {missing.map((type) => (
          <Button key={type} variant="outline" size="sm" onClick={() => onChange([...value, type])}>
            <Plus /> {questionTypeLabel[type]}
          </Button>
        ))}
      </div>
    </div>
  )
}
