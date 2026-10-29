import { useId } from 'react'
import type { AiSettings } from '@/bindings'
import { Input } from '@/components/ui/input'
import { wholeSeconds, type ModelsDraft } from '@/features/models/models-draft'

type Props = {
  draft: ModelsDraft
  defaults: AiSettings
  onDraft: (draft: ModelsDraft) => void
}

/** How long an executor may sit unused, and how long ssh may take to connect. */
export function TimeoutsSection({ draft, defaults, onDraft }: Props) {
  return (
    <section className="flex flex-col">
      <h2 className="mb-1 font-semibold">超时</h2>
      <div className="flex flex-col divide-y">
        <SecondsRow
          label="闲置超时"
          value={draft.idle}
          defaultSeconds={defaults.invoker.idle_timeout_secs}
          onChange={(idle) => onDraft({ ...draft, idle })}
        />
        <SecondsRow
          label="连接超时"
          value={draft.connect}
          defaultSeconds={defaults.invoker.connect_timeout_secs}
          onChange={(connect) => onDraft({ ...draft, connect })}
        />
      </div>
    </section>
  )
}

type RowProps = {
  label: string
  value: string
  defaultSeconds: number
  onChange: (value: string) => void
}

function SecondsRow({ label, value, defaultSeconds, onChange }: RowProps) {
  const id = useId()
  return (
    <div className="grid grid-cols-[9rem_1fr] items-center gap-x-4 py-2">
      <label htmlFor={id} className="text-sm text-muted-foreground">
        {label}
      </label>
      <div className="flex min-h-8 items-center gap-2 text-sm">
        <Input
          id={id}
          inputMode="numeric"
          value={value}
          aria-invalid={wholeSeconds(value) === null}
          className="h-8 w-24"
          onChange={(event) => onChange(event.target.value)}
        />
        <span className="text-muted-foreground">秒，默认 {defaultSeconds}</span>
      </div>
    </div>
  )
}
