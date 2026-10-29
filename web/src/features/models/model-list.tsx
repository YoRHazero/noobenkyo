import { useId } from 'react'
import type { MachineReport } from '@/bindings'
import { Checkbox } from '@/components/ui/checkbox'
import { opNames, runtimeNames } from '@/features/models/model-names'
import { modelRows, tickModel, type ModelRow } from '@/features/models/model-ticks'
import type { MachineDraft } from '@/features/models/models-draft'

type Props = {
  machine: MachineDraft
  report: MachineReport | null
  onChange: (machine: MachineDraft) => void
}

/**
 * The models the machine may run: what its probe found, ticked or not, and
 * what is ticked but was not found this time. Only what was found can be
 * ticked; what is ticked can always be unticked.
 */
export function ModelList({ machine, report, onChange }: Props) {
  const rows = modelRows(machine, report)
  if (rows.length === 0) {
    return report?.kind === 'online' ? (
      <p className="text-sm text-muted-foreground">无模型</p>
    ) : null
  }
  return (
    <ul className="flex flex-col gap-1.5">
      {rows.map((row) => (
        <ModelItem
          key={`${row.kind}/${row.path}`}
          row={row}
          onTick={(on) => onChange(tickModel(machine, report, row, on))}
        />
      ))}
    </ul>
  )
}

function ModelItem({ row, onTick }: { row: ModelRow; onTick: (on: boolean) => void }) {
  const id = useId()
  return (
    <li className="flex items-center gap-2 text-sm">
      <Checkbox
        id={id}
        checked={row.ticked}
        disabled={!row.ticked && row.found !== true}
        onCheckedChange={(value) => onTick(value === true)}
      />
      <label htmlFor={id}>{row.name}</label>
      <span className="text-xs text-muted-foreground">
        {runtimeNames[row.kind]}
        {row.op && ` · ${opNames[row.op]}`}
      </span>
      {row.found === false && <span className="text-xs text-destructive">未找到</span>}
    </li>
  )
}
