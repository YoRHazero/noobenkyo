import { describe, expect, it } from 'vitest'
import { modelOptions } from '@/features/models/model-options'
import { llamaCpp, qwenTts, yorha } from '@/features/models/test-machines'

describe('the models a use may pick', () => {
  it('lists the ticked qwen-tts models taking the call, each once', () => {
    const machines = [yorha([qwenTts, llamaCpp]), { ...yorha([qwenTts]), key: 1, name: 'zero' }]
    expect(modelOptions(machines, 'TtsClone')).toEqual(['Base'])
    expect(modelOptions(machines, 'TtsDesign')).toEqual(['VoiceDesign'])
    expect(modelOptions([yorha()], 'TtsClone')).toEqual([])
  })
})
