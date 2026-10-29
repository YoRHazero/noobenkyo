import type { MachineReport, QwenTtsModel, Runtime } from '@/bindings'
import type { MachineDraft } from '@/features/models/models-draft'

/** Shared by this feature's tests: yorha, and what probing it finds. */
export const base: QwenTtsModel = { name: 'Base', op: 'TtsClone', path: '/models/Base' }

export const voiceDesign: QwenTtsModel = {
  name: 'VoiceDesign',
  op: 'TtsDesign',
  path: '/models/VoiceDesign',
}

export const qwenTts: Extract<Runtime, { kind: 'QwenTts' }> = {
  kind: 'QwenTts',
  python: '/opt/qwen-tts/.venv/bin/python',
  models: [base, voiceDesign],
}

export const llamaCpp: Extract<Runtime, { kind: 'LlamaCpp' }> = {
  kind: 'LlamaCpp',
  python: '/opt/qwen-tts/.venv/bin/python',
  server: '/opt/llama.cpp/bin/llama-server',
  models: [{ name: 'Small-Q8_0', path: '/models/Small/Small-Q8_0.gguf', args: ['-c', '16384'] }],
}

export const online: MachineReport = {
  kind: 'online',
  models_dir: true,
  qwen_tts: { runtime: qwenTts, problem: null },
  llama_cpp: { runtime: llamaCpp, problem: null },
}

export function yorha(runtimes: Runtime[] = []): MachineDraft {
  return {
    key: 0,
    name: 'yorha',
    ssh: 'yorha@192.0.2.10',
    prefixes: ['wsl -e'],
    models: '/models',
    qwenTts: '/opt/qwen-tts',
    llamaCpp: '/opt/llama.cpp',
    llamaPython: '',
    runtimes,
  }
}
