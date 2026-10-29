import { useState } from 'react'
import { ArrowDown, ArrowUp, Trash2 } from 'lucide-react'
import type { MachineReport } from '@/bindings'
import { useAiProbeMachine } from '@/backend/ai/use-ai-probe-machine'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { MachineFields } from '@/features/models/machine-fields'
import { MachineStatus } from '@/features/models/machine-status'
import { ModelList } from '@/features/models/model-list'
import { toMachine, type MachineDraft } from '@/features/models/models-draft'

type Props = {
  machine: MachineDraft
  /** The last probe of all the saved machines found for this name; `null` if none. */
  report: MachineReport | null
  reportsAt: number
  /** Every saved machine is being probed again, this one among them. */
  probingAll: boolean
  first: boolean
  last: boolean
  onChange: (change: Partial<Omit<MachineDraft, 'key'>>) => void
  onMove: (by: -1 | 1) => void
  onRemove: () => void
  /** 检查 came back with this; the form takes over what it found. */
  onChecked: (report: MachineReport) => void
}

/**
 * One machine: how to reach it, where to look on it, what was found there,
 * and the models it may run. 检查 probes it as the form has it; that report
 * shows until the next probe of all the machines, and ticked models found at
 * the same path take the names they were found under.
 */
export function MachineCard({ machine, report, reportsAt, probingAll, ...on }: Props) {
  const probe = useAiProbeMachine()
  const [own, setOwn] = useState<{ report: MachineReport; at: number } | null>(null)
  const shown = own && own.at > reportsAt ? own.report : report
  const check = () => {
    setOwn(null)
    probe.mutate(toMachine(machine), {
      onSuccess: (checked) => {
        setOwn({ report: checked, at: Date.now() })
        on.onChecked(checked)
      },
    })
  }

  return (
    <div className="flex flex-col gap-3 rounded-xl border px-4 py-3">
      <div className="flex items-center gap-2">
        <Input
          aria-label="机器名"
          placeholder="机器名"
          value={machine.name}
          className="h-8 w-48 font-medium"
          onChange={(event) => on.onChange({ name: event.target.value })}
        />
        <MachineStatus
          report={shown}
          checking={probe.isPending || probingAll}
          error={probe.error}
        />
        <div className="ml-auto flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="上移"
            disabled={on.first}
            onClick={() => on.onMove(-1)}
          >
            <ArrowUp />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="下移"
            disabled={on.last}
            onClick={() => on.onMove(1)}
          >
            <ArrowDown />
          </Button>
          <Button variant="outline" size="sm" disabled={probe.isPending} onClick={check}>
            检查
          </Button>
          <Button variant="ghost" size="icon-sm" aria-label="删除机器" onClick={on.onRemove}>
            <Trash2 />
          </Button>
        </div>
      </div>
      <MachineFields machine={machine} onChange={on.onChange} />
      <ModelList
        machine={machine}
        report={shown}
        onChange={(changed) => on.onChange({ runtimes: changed.runtimes })}
      />
    </div>
  )
}
