import type { QuestionType } from '@/bindings'
import { Plus, X } from 'lucide-react'
import { Reorder } from 'motion/react'
import { questionTypeName } from '@/features/settings/question-type-names'

type Props = {
  value: QuestionType[]
  /** The types this order may use, in the order unused ones are offered. */
  types: QuestionType[]
  onChange: (value: QuestionType[]) => void
}

/**
 * A question order as chips: drag one to move it, × to take it out. The
 * types left out follow as dashed chips; clicking one adds it at the end.
 * An empty order is the backend's to refuse.
 */
export function QuestionOrderEditor({ value, types, onChange }: Props) {
  const unused = types.filter((type) => !value.includes(type))
  return (
    <div className="flex flex-wrap items-center gap-2">
      <Reorder.Group axis="x" values={value} onReorder={onChange} className="flex gap-2">
        {value.map((type, index) => (
          <Reorder.Item
            key={type}
            value={type}
            whileDrag={{ scale: 1.06, boxShadow: '0 6px 16px rgb(0 0 0 / 0.15)' }}
            className="flex cursor-grab items-center gap-1.5 rounded-md border bg-card py-1 pr-1 pl-2.5 text-sm shadow-xs select-none active:cursor-grabbing"
          >
            <span className="text-xs text-muted-foreground tabular-nums">{index + 1}</span>
            {questionTypeName[type]}
            <button
              type="button"
              aria-label={`去掉${questionTypeName[type]}`}
              className="rounded-sm p-0.5 text-muted-foreground hover:bg-accent hover:text-foreground"
              // A press on × is not the start of a drag.
              onPointerDown={(event) => event.stopPropagation()}
              onClick={() => onChange(value.filter((t) => t !== type))}
            >
              <X className="size-3.5" />
            </button>
          </Reorder.Item>
        ))}
      </Reorder.Group>
      {unused.map((type) => (
        <button
          key={type}
          type="button"
          className="flex items-center gap-1 rounded-md border border-dashed px-2.5 py-1 text-sm text-muted-foreground hover:bg-accent hover:text-foreground"
          onClick={() => onChange([...value, type])}
        >
          <Plus className="size-3.5" />
          {questionTypeName[type]}
        </button>
      ))}
    </div>
  )
}
