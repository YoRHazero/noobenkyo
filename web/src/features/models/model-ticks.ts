import type { MachineReport, Runtime, TtsOp } from '@/bindings'
import type { MachineDraft } from '@/features/models/models-draft'

export type RuntimeKind = Runtime['kind']

/**
 * A model shown under a machine: found by its probe, ticked in its settings,
 * or both. Entries in the same runtime with the same path are one model,
 * whatever they are named.
 */
export type ModelRow = {
  kind: RuntimeKind
  name: string
  path: string
  /** Which call a qwen-tts model takes; `null` for llama.cpp's. */
  op: TtsOp | null
  ticked: boolean
  /** Whether the probe found it; `null` when it did not look in this runtime. */
  found: boolean | null
}

/** The runtimes a probe looked in, each with every model found there. */
export function foundRuntimes(report: MachineReport | null): Runtime[] {
  if (report?.kind !== 'online') {
    return []
  }
  return [report.qwen_tts, report.llama_cpp].flatMap((found) => (found ? [found.runtime] : []))
}

/**
 * What the probe found, ticked or not, then what is ticked but was not
 * found where the probe looked, or was not looked for.
 */
export function modelRows(machine: MachineDraft, report: MachineReport | null): ModelRow[] {
  const found = foundRuntimes(report)
  const ticked = (kind: RuntimeKind, path: string) =>
    machine.runtimes.some((runtime) => runtime.kind === kind && hasPath(runtime, path))
  const rows: ModelRow[] = found.flatMap((runtime) =>
    entriesOf(runtime).map((model) => ({
      kind: runtime.kind,
      ...model,
      ticked: ticked(runtime.kind, model.path),
      found: true,
    })),
  )
  for (const runtime of machine.runtimes) {
    const looked = found.find((other) => other.kind === runtime.kind)
    for (const model of entriesOf(runtime)) {
      if (!looked || !hasPath(looked, model.path)) {
        rows.push({ kind: runtime.kind, ...model, ticked: true, found: looked ? false : null })
      }
    }
  }
  return rows
}

/**
 * The machine with the model ticked or not. Ticking takes the model, and its
 * runtime's python and server, from what the probe found; a runtime left
 * without models goes.
 */
export function tickModel(
  machine: MachineDraft,
  report: MachineReport | null,
  row: ModelRow,
  on: boolean,
): MachineDraft {
  const kept = machine.runtimes.find((runtime) => runtime.kind === row.kind)
  if (!on) {
    const runtimes = machine.runtimes.flatMap((runtime) => {
      if (runtime !== kept) {
        return [runtime]
      }
      const left = withoutPath(runtime, row.path)
      return left.models.length === 0 ? [] : [left]
    })
    return { ...machine, runtimes }
  }
  const found = foundRuntimes(report).find((runtime) => runtime.kind === row.kind)
  if (!found || !hasPath(found, row.path)) {
    return machine
  }
  const next = withModel(found, kept, row.path)
  const runtimes = kept
    ? machine.runtimes.map((runtime) => (runtime === kept ? next : runtime))
    : [...machine.runtimes, next]
  return { ...machine, runtimes }
}

function entriesOf(runtime: Runtime): { name: string; path: string; op: TtsOp | null }[] {
  return runtime.kind === 'QwenTts'
    ? runtime.models.map(({ name, path, op }) => ({ name, path, op }))
    : runtime.models.map(({ name, path }) => ({ name, path, op: null }))
}

function hasPath(runtime: Runtime, path: string): boolean {
  return runtime.models.some((model) => model.path === path)
}

function withoutPath(runtime: Runtime, path: string): Runtime {
  return runtime.kind === 'QwenTts'
    ? { ...runtime, models: runtime.models.filter((model) => model.path !== path) }
    : { ...runtime, models: runtime.models.filter((model) => model.path !== path) }
}

/** `found`'s python and server, with the models already ticked and the one at `path`. */
function withModel(found: Runtime, kept: Runtime | undefined, path: string): Runtime {
  if (found.kind === 'QwenTts') {
    const before = kept?.kind === 'QwenTts' ? kept.models : []
    const added = found.models.filter((model) => model.path === path)
    return { ...found, models: [...before.filter((model) => model.path !== path), ...added] }
  }
  const before = kept?.kind === 'LlamaCpp' ? kept.models : []
  const added = found.models.filter((model) => model.path === path)
  return { ...found, models: [...before.filter((model) => model.path !== path), ...added] }
}
