import { describe, expect, it } from 'vitest'
import type { MachineReport, Runtime } from '@/bindings'
import { modelRows, tickModel, type ModelRow } from '@/features/models/model-ticks'
import {
  base,
  llamaCpp,
  online,
  qwenTts,
  voiceDesign,
  yorha,
} from '@/features/models/test-machines'

const row = (rows: ModelRow[], name: string) => rows.find((r) => r.name === name) as ModelRow

/** An old hand-written entry: another name for the model at /models/Base. */
const oldBase = { name: 'qwen3-tts-1.7b-base', op: 'TtsClone', path: '/models/Base' } as const

describe('ticking the models a machine may run', () => {
  it('lists what was found, then what is ticked but was not found', () => {
    const old: Runtime = {
      kind: 'QwenTts',
      python: '/old/python',
      models: [base, { name: 'Gone', op: 'TtsClone', path: '/models/Gone' }],
    }
    const rows = modelRows(yorha([old]), online)
    expect(rows.map((r) => [r.name, r.op, r.ticked, r.found])).toEqual([
      ['Base', 'TtsClone', true, true],
      ['VoiceDesign', 'TtsDesign', false, true],
      ['Small-Q8_0', null, false, true],
      ['Gone', 'TtsClone', true, false],
    ])
  })

  it('takes an entry under another name at the same path for the model found', () => {
    const old: Runtime = { kind: 'QwenTts', python: '/old/python', models: [oldBase] }
    const rows = modelRows(yorha([old]), online)
    expect(rows.map((r) => [r.name, r.ticked, r.found])).toEqual([
      ['Base', true, true],
      ['VoiceDesign', false, true],
      ['Small-Q8_0', false, true],
    ])
  })

  it('cannot tell what was found where the probe did not look', () => {
    expect(modelRows(yorha([qwenTts]), null).map((r) => r.found)).toEqual([null, null])
    const offline: MachineReport = { kind: 'offline', reason: '连接失败' }
    expect(modelRows(yorha([qwenTts]), offline)[0]?.found).toBeNull()
    // No qwen-tts directory given: it was not looked for, so not missing.
    const llamaOnly: MachineReport = { ...online, qwen_tts: null } as MachineReport
    expect(modelRows(yorha([qwenTts]), llamaOnly).map((r) => [r.name, r.found])).toEqual([
      ['Small-Q8_0', true],
      ['Base', null],
      ['VoiceDesign', null],
    ])
  })

  it('ticks a model with its runtime as the probe found it', () => {
    const machine = tickModel(yorha(), online, row(modelRows(yorha(), online), 'VoiceDesign'), true)
    expect(machine.runtimes).toEqual([{ ...qwenTts, models: [voiceDesign] }])
    const both = tickModel(machine, online, row(modelRows(machine, online), 'Small-Q8_0'), true)
    expect(both.runtimes[1]).toEqual(llamaCpp)
  })

  it('takes the python of the latest probe for a runtime already there', () => {
    const old: Runtime = { ...qwenTts, python: '/old/python', models: [base] }
    const rows = modelRows(yorha([old]), online)
    const machine = tickModel(yorha([old]), online, row(rows, 'VoiceDesign'), true)
    expect(machine.runtimes).toEqual([qwenTts])
  })

  it('unticks an entry by its path, whatever it is named', () => {
    const old: Runtime = { kind: 'QwenTts', python: '/old/python', models: [oldBase] }
    const machine = yorha([old, llamaCpp])
    const none = tickModel(machine, online, row(modelRows(machine, online), 'Base'), false)
    expect(none.runtimes).toEqual([llamaCpp])
  })

  it('drops a runtime left without models', () => {
    const machine = yorha([qwenTts, llamaCpp])
    const rows = modelRows(machine, online)
    const less = tickModel(machine, online, row(rows, 'Base'), false)
    expect(less.runtimes[0]).toEqual({ ...qwenTts, models: [voiceDesign] })
    const none = tickModel(less, online, row(rows, 'VoiceDesign'), false)
    expect(none.runtimes).toEqual([llamaCpp])
  })

  it('cannot tick what the probe did not find', () => {
    const machine = yorha()
    const gone: ModelRow = {
      kind: 'QwenTts',
      name: 'Gone',
      path: '/models/Gone',
      op: 'TtsClone',
      ticked: false,
      found: false,
    }
    expect(tickModel(machine, online, gone, true)).toBe(machine)
    expect(tickModel(machine, null, row(modelRows(machine, online), 'Base'), true)).toBe(machine)
  })
})
