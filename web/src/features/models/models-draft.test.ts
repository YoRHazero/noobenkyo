import { describe, expect, it } from 'vitest'
import type { AiSettings } from '@/bindings'
import {
  fromDraft,
  sameSettings,
  toDraft,
  toMachine,
  wholeSeconds,
} from '@/features/models/models-draft'
import { qwenTts, yorha } from '@/features/models/test-machines'

const settings: AiSettings = {
  invoker: {
    machines: [
      {
        name: 'yorha',
        ssh: 'yorha@192.0.2.10',
        command_layers: ['wsl -e'],
        runtimes: [qwenTts],
        probe: {
          models: '/models',
          qwen_tts: '/opt/qwen-tts',
          llama_cpp: '/opt/llama.cpp',
          llama_python: null,
        },
      },
    ],
    idle_timeout_secs: 600,
    connect_timeout_secs: 5,
  },
  models: { speak: 'Base', design_voice: null },
}

describe('the model page draft', () => {
  it('gives back the settings it was made from', () => {
    expect(fromDraft(toDraft(settings))).toEqual(settings)
  })

  it('leaves out what is blank and trims the rest', () => {
    const machine = {
      ...yorha(),
      name: ' zero ',
      ssh: '  ',
      prefixes: [' wsl -e ', ''],
      qwenTts: '',
      llamaPython: '/usr/local/bin/python3',
    }
    expect(toMachine(machine)).toEqual({
      name: 'zero',
      ssh: null,
      command_layers: ['wsl -e'],
      runtimes: [],
      probe: {
        models: '/models',
        qwen_tts: null,
        llama_cpp: '/opt/llama.cpp',
        llama_python: '/usr/local/bin/python3',
      },
    })
  })

  it('keeps no python for llama.cpp beside qwen-tts, and no probe without directories', () => {
    expect(
      toMachine({ ...yorha(), llamaPython: '/usr/local/bin/python3' }).probe?.llama_python,
    ).toBe(null)
    const bare = { ...yorha(), models: '', qwenTts: '', llamaCpp: '', llamaPython: ' ' }
    expect(toMachine(bare).probe).toBeNull()
  })

  it('needs whole seconds from 1', () => {
    expect(wholeSeconds(' 600 ')).toBe(600)
    expect(wholeSeconds('0')).toBeNull()
    expect(wholeSeconds('1.5')).toBeNull()
    expect(wholeSeconds('')).toBeNull()
    expect(fromDraft({ ...toDraft(settings), connect: 'x' })).toBeNull()
  })

  it('compares settings by what they would save', () => {
    const reordered: AiSettings = { models: settings.models, invoker: settings.invoker }
    expect(sameSettings(settings, reordered)).toBe(true)
    const other = { ...settings, models: { speak: null, design_voice: null } }
    expect(sameSettings(settings, other)).toBe(false)
  })
})
