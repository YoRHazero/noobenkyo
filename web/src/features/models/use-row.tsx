import type { TtsOp } from '@/bindings'
import { opNames } from '@/features/models/model-names'

type Props = {
  label: string
  op: TtsOp
  /** The models some machine has ticked for this use. */
  options: string[]
  value: string | null
  onChange: (model: string | null) => void
}

/** One use and its model. A pick that no machine has ticked any more stays, marked. */
export function UseRow({ label, op, options, value, onChange }: Props) {
  const lost = value !== null && !options.includes(value)
  return (
    <div className="grid grid-cols-[9rem_1fr] items-center gap-x-4 py-2">
      <span className="text-sm text-muted-foreground">{label}</span>
      <div className="flex min-h-8 flex-wrap items-center gap-x-3 gap-y-1">
        <select
          aria-label={label}
          value={value ?? ''}
          className="h-8 w-80 rounded-md border border-input bg-transparent px-2 text-sm"
          onChange={(event) => onChange(event.target.value === '' ? null : event.target.value)}
        >
          <option value="">未选</option>
          {lost && <option value={value}>{value}</option>}
          {options.map((model) => (
            <option key={model} value={model}>
              {model}
            </option>
          ))}
        </select>
        {lost ? (
          <span className="text-xs text-destructive">无机器勾选</span>
        ) : (
          options.length === 0 && (
            <span className="text-xs text-muted-foreground">无{opNames[op]}模型</span>
          )
        )}
      </div>
    </div>
  )
}
