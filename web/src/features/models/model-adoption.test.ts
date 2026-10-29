import { describe, expect, it } from 'vitest'
import type { Runtime } from '@/bindings'
import { adoptFound, adoptReport } from '@/features/models/model-adoption'
import type { ModelsDraft } from '@/features/models/models-draft'
import { llamaCpp, online, qwenTts, voiceDesign, yorha } from '@/features/models/test-machines'

/** yorha as hand-written before the model page: other names, the same paths. */
const handWritten: Runtime[] = [
  {
    kind: 'QwenTts',
    python: '/old/python',
    models: [
      { name: 'qwen3-tts-1.7b-base', op: 'TtsClone', path: '/models/Base' },
      { name: 'Gone', op: 'TtsDesign', path: '/models/Gone' },
    ],
  },
  {
    kind: 'LlamaCpp',
    python: '/usr/bin/python3',
    server: '/old/llama-server',
    models: [{ name: 'small', path: '/models/Small/Small-Q8_0.gguf', args: ['-t', '8'] }],
  },
]

function draft(speak: string | null, others: Runtime[][] = []): ModelsDraft {
  return {
    machines: [
      yorha(handWritten),
      ...others.map((runtimes, i) => ({ ...yorha(runtimes), key: i + 1 })),
    ],
    speak,
    designVoice: null,
    idle: '600',
    connect: '5',
  }
}

describe('taking over what a check found', () => {
  it('renames entries found at the same path and takes the python and server found', () => {
    const { machine, renamed } = adoptFound(yorha(handWritten), online)
    expect(machine.runtimes).toEqual([
      {
        ...qwenTts,
        models: [qwenTts.models[0], { name: 'Gone', op: 'TtsDesign', path: '/models/Gone' }],
      },
      { ...llamaCpp, models: [{ ...llamaCpp.models[0], args: ['-t', '8'] }] },
    ])
    expect(renamed).toEqual([
      { from: 'qwen3-tts-1.7b-base', to: 'Base' },
      { from: 'small', to: 'Small-Q8_0' },
    ])
  })

  it('leaves a runtime alone when nothing in it was found', () => {
    const elsewhere: Runtime = {
      ...qwenTts,
      python: '/old/python',
      models: [{ ...voiceDesign, path: '/elsewhere' }],
    }
    expect(adoptFound(yorha([elsewhere]), online).machine.runtimes).toEqual([elsewhere])
    expect(adoptFound(yorha(handWritten), { kind: 'offline', reason: '连接失败' }).renamed).toEqual(
      [],
    )
  })

  it('moves a use to the new name, unless another machine still has the old one', () => {
    expect(adoptReport(draft('qwen3-tts-1.7b-base'), 0, online).speak).toBe('Base')
    expect(adoptReport(draft('Gone'), 0, online).speak).toBe('Gone')
    const shared = draft('qwen3-tts-1.7b-base', [handWritten])
    expect(adoptReport(shared, 0, online).speak).toBe('qwen3-tts-1.7b-base')
  })
})
