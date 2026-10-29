import type { MachineReport, Runtime } from '@/bindings'
import { foundRuntimes } from '@/features/models/model-ticks'
import type { MachineDraft, ModelsDraft } from '@/features/models/models-draft'

/** A model a check renamed: its name before, and the name it was found under. */
export type Renamed = { from: string; to: string }

/**
 * The machine after a check: each ticked model found at the same path takes
 * the name it was found under, and its runtime the python and server the
 * check found. A llama.cpp model keeps its arguments.
 */
export function adoptFound(
  machine: MachineDraft,
  report: MachineReport,
): { machine: MachineDraft; renamed: Renamed[] } {
  const renamed: Renamed[] = []
  const found = foundRuntimes(report)
  const runtimes = machine.runtimes.map((runtime) => {
    const looked = found.find((other) => other.kind === runtime.kind)
    return looked ? adoptRuntime(runtime, looked, renamed) : runtime
  })
  return { machine: { ...machine, runtimes }, renamed }
}

/**
 * The draft after checking one of its machines. A use picked under a name
 * the check changed follows it, unless another machine still has that name.
 */
export function adoptReport(draft: ModelsDraft, key: number, report: MachineReport): ModelsDraft {
  const machine = draft.machines.find((other) => other.key === key)
  if (!machine) {
    return draft
  }
  const adopted = adoptFound(machine, report)
  const machines = draft.machines.map((other) => (other.key === key ? adopted.machine : other))
  const named = (name: string) =>
    machines.some((other) =>
      other.runtimes.some((runtime) => runtime.models.some((model) => model.name === name)),
    )
  const follow = (chosen: string | null) => {
    const to = adopted.renamed.find((rename) => rename.from === chosen)?.to
    return chosen !== null && to !== undefined && !named(chosen) ? to : chosen
  }
  return { ...draft, machines, speak: follow(draft.speak), designVoice: follow(draft.designVoice) }
}

function adoptRuntime(saved: Runtime, found: Runtime, renamed: Renamed[]): Runtime {
  const rename = (from: string, to: string) => {
    if (from !== to) {
      renamed.push({ from, to })
    }
  }
  if (saved.kind === 'QwenTts' && found.kind === 'QwenTts') {
    if (!saved.models.some((model) => found.models.some((other) => other.path === model.path))) {
      return saved
    }
    const models = saved.models.map((model) => {
      const same = found.models.find((other) => other.path === model.path)
      if (!same) {
        return model
      }
      rename(model.name, same.name)
      return same
    })
    return { ...found, models }
  }
  if (saved.kind === 'LlamaCpp' && found.kind === 'LlamaCpp') {
    if (!saved.models.some((model) => found.models.some((other) => other.path === model.path))) {
      return saved
    }
    const models = saved.models.map((model) => {
      const same = found.models.find((other) => other.path === model.path)
      if (!same) {
        return model
      }
      rename(model.name, same.name)
      return { ...same, args: model.args }
    })
    return { ...found, models }
  }
  return saved
}
