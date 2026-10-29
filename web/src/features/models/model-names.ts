import type { TtsOp } from '@/bindings'
import type { RuntimeKind } from '@/features/models/model-ticks'

export const runtimeNames: Record<RuntimeKind, string> = {
  QwenTts: 'qwen-tts',
  LlamaCpp: 'llama.cpp',
}

export const opNames: Record<TtsOp, string> = { TtsClone: '克隆', TtsDesign: '设计' }
