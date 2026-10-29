import type { AiSettings, Machine, Runtime } from '@/bindings'

/** One machine as the form edits it: text as typed, '' for what is left out. */
export type MachineDraft = {
  /** Stays while the page is open: React's key, and the card's own probe. */
  key: number
  name: string
  ssh: string
  /** Command prefixes, outermost first, such as `wsl -e`. */
  prefixes: string[]
  models: string
  qwenTts: string
  llamaCpp: string
  /** Only used when there is no qwen-tts. */
  llamaPython: string
  /** The models ticked for it, as the settings keep them. */
  runtimes: Runtime[]
}

export type ModelsDraft = {
  machines: MachineDraft[]
  speak: string | null
  designVoice: string | null
  /** As typed: whole seconds from 1. */
  idle: string
  connect: string
}

export function toDraft(settings: AiSettings): ModelsDraft {
  return {
    machines: settings.invoker.machines.map((machine, key) => ({
      key,
      name: machine.name,
      ssh: machine.ssh ?? '',
      prefixes: machine.command_layers,
      models: machine.probe?.models ?? '',
      qwenTts: machine.probe?.qwen_tts ?? '',
      llamaCpp: machine.probe?.llama_cpp ?? '',
      llamaPython: machine.probe?.llama_python ?? '',
      runtimes: machine.runtimes,
    })),
    speak: settings.models.speak,
    designVoice: settings.models.design_voice,
    idle: String(settings.invoker.idle_timeout_secs),
    connect: String(settings.invoker.connect_timeout_secs),
  }
}

/** The settings to save; `null` while a time is not a whole number of seconds. */
export function fromDraft(draft: ModelsDraft): AiSettings | null {
  const idle = wholeSeconds(draft.idle)
  const connect = wholeSeconds(draft.connect)
  if (idle === null || connect === null) {
    return null
  }
  return {
    invoker: {
      machines: draft.machines.map(toMachine),
      idle_timeout_secs: idle,
      connect_timeout_secs: connect,
    },
    models: { speak: draft.speak, design_voice: draft.designVoice },
  }
}

/** The machine as the backend takes it, to save or to probe. */
export function toMachine(machine: MachineDraft): Machine {
  const qwenTts = optional(machine.qwenTts)
  const dirs = [machine.models, machine.qwenTts, machine.llamaCpp, machine.llamaPython]
  return {
    name: machine.name.trim(),
    ssh: optional(machine.ssh),
    command_layers: machine.prefixes.map((prefix) => prefix.trim()).filter((p) => p !== ''),
    runtimes: machine.runtimes,
    probe: dirs.every((dir) => dir.trim() === '')
      ? null
      : {
          models: machine.models.trim(),
          qwen_tts: qwenTts,
          llama_cpp: optional(machine.llamaCpp),
          // llama.cpp runs on qwen-tts's python when there is one.
          llama_python: qwenTts === null ? optional(machine.llamaPython) : null,
        },
  }
}

/** A whole number from 1, as typed; `null` otherwise. */
export function wholeSeconds(text: string): number | null {
  const trimmed = text.trim()
  if (!/^\d+$/.test(trimmed)) {
    return null
  }
  const value = Number(trimmed)
  return value >= 1 ? value : null
}

/** Whether two settings would save the same, field order aside. */
export function sameSettings(a: AiSettings, b: AiSettings): boolean {
  const draftA = fromDraft(toDraft(a))
  const draftB = fromDraft(toDraft(b))
  return JSON.stringify(draftA) === JSON.stringify(draftB)
}

function optional(text: string): string | null {
  const trimmed = text.trim()
  return trimmed === '' ? null : trimmed
}
