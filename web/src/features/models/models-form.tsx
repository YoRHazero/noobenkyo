import { useState } from 'react'
import type { AiSettings, MachineProbe } from '@/bindings'
import { useAiUpdateSettings } from '@/backend/ai/use-ai-update-settings'
import { Button } from '@/components/ui/button'
import { MachineCard } from '@/features/models/machine-card'
import { adoptReport } from '@/features/models/model-adoption'
import {
  addMachine,
  changeMachine,
  moveMachine,
  removeMachine,
} from '@/features/models/machine-edits'
import { fromDraft, sameSettings, toDraft } from '@/features/models/models-draft'
import { ModelsSaveBar } from '@/features/models/models-save-bar'
import { TimeoutsSection } from '@/features/models/timeouts-section'
import { UsesSection } from '@/features/models/uses-section'

type Props = {
  saved: AiSettings
  defaults: AiSettings
  /** The last probe of the saved machines, by name. */
  reports: MachineProbe[]
  /** When those reports came. */
  reportsAt: number
  probingAll: boolean
  /** The settings were saved: what the last probe found may not hold for them. */
  onSaved: () => void
}

/**
 * Every ai setting, saved together as one replacement; nothing is saved
 * until 保存. A refused save keeps the form.
 */
export function ModelsForm({ saved, defaults, reports, reportsAt, probingAll, onSaved }: Props) {
  const [draft, setDraft] = useState(() => toDraft(saved))
  const update = useAiUpdateSettings()
  const next = fromDraft(draft)
  const setMachines = (machines: typeof draft.machines) => setDraft({ ...draft, machines })
  const savedNames = saved.invoker.machines.map((machine) => machine.name)

  return (
    <div className="flex flex-col gap-8">
      <UsesSection draft={draft} onDraft={setDraft} />
      <section className="flex flex-col gap-3">
        <h2 className="font-semibold">机器</h2>
        {draft.machines.map((machine, index) => (
          <MachineCard
            key={machine.key}
            machine={machine}
            report={reports.find((probe) => probe.machine === machine.name)?.report ?? null}
            reportsAt={reportsAt}
            probingAll={probingAll && savedNames.includes(machine.name)}
            first={index === 0}
            last={index === draft.machines.length - 1}
            onChange={(changed) => setMachines(changeMachine(draft.machines, machine.key, changed))}
            onMove={(by) => setMachines(moveMachine(draft.machines, machine.key, by))}
            onRemove={() => setMachines(removeMachine(draft.machines, machine.key))}
            // The check ends later; the draft may have changed meanwhile.
            onChecked={(report) => setDraft((now) => adoptReport(now, machine.key, report))}
          />
        ))}
        <Button
          variant="outline"
          size="sm"
          className="self-start"
          onClick={() => setMachines(addMachine(draft.machines))}
        >
          添加机器
        </Button>
      </section>
      <TimeoutsSection draft={draft} defaults={defaults} onDraft={setDraft} />
      <ModelsSaveBar
        changed={next === null || !sameSettings(next, saved)}
        complete={next !== null}
        busy={update.isPending}
        error={update.error}
        onRevert={() => setDraft(toDraft(saved))}
        onSave={() => next && update.mutate(next, { onSuccess: onSaved })}
      />
    </div>
  )
}
