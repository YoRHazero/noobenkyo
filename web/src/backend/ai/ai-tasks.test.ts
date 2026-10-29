import { beforeEach, describe, expect, it } from 'vitest'
import {
  aiTaskFinished,
  aiTaskProgressed,
  aiTaskStarted,
  speakLinesTask,
  useAiTasks,
} from '@/backend/ai/ai-tasks'

const latest = (episode: string) => speakLinesTask(useAiTasks.getState(), episode)
const speakLines = (episode: string) => ({ kind: 'speak_lines' as const, episode })

describe('ai tasks', () => {
  beforeEach(() => useAiTasks.setState({ tasks: [] }))

  it('follows a task from its start to its end', () => {
    aiTaskStarted(1, speakLines('a'))
    expect(latest('a')).toEqual({
      id: 1,
      subject: speakLines('a'),
      done: 0,
      total: null,
      outcome: null,
    })
    aiTaskProgressed({ task: 1, done: 3, total: 16 })
    aiTaskFinished({ task: 1, outcome: { kind: 'cancelled' } })
    expect(latest('a')).toMatchObject({ done: 3, total: 16, outcome: { kind: 'cancelled' } })
  })

  it('keeps events that come before the command returns', () => {
    aiTaskProgressed({ task: 1, done: 0, total: 16 })
    aiTaskFinished({ task: 1, outcome: { kind: 'failed', message: '没有机器' } })
    expect(latest('a')).toBeUndefined()
    aiTaskStarted(1, speakLines('a'))
    expect(latest('a')).toMatchObject({
      total: 16,
      outcome: { kind: 'failed', message: '没有机器' },
    })
  })

  it('keeps only the latest task on an episode', () => {
    aiTaskStarted(1, speakLines('a'))
    aiTaskFinished({ task: 1, outcome: { kind: 'failed', message: '没有机器' } })
    aiTaskStarted(2, speakLines('b'))
    aiTaskStarted(3, speakLines('a'))
    expect(latest('a')).toMatchObject({ id: 3, outcome: null })
    expect(latest('b')).toMatchObject({ id: 2 })
    expect(useAiTasks.getState().tasks).toHaveLength(2)
  })
})
