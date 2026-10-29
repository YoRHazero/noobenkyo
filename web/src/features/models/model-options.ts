import type { TtsOp } from '@/bindings'
import type { MachineDraft } from '@/features/models/models-draft'

/** What a use may pick: the qwen-tts models taking `op` that some machine has ticked, each once. */
export function modelOptions(machines: MachineDraft[], op: TtsOp): string[] {
  const names = machines.flatMap((machine) =>
    machine.runtimes.flatMap((runtime) =>
      runtime.kind === 'QwenTts'
        ? runtime.models.filter((model) => model.op === op).map((model) => model.name)
        : [],
    ),
  )
  return [...new Set(names)]
}
