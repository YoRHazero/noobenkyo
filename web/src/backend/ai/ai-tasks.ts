import { create } from 'zustand'
import type { AiTaskFinished, AiTaskProgress, TaskId, TaskOutcome } from '@/bindings'

/** What a task works on, as the frontend started it. */
export type AiTaskSubject = { kind: 'speak_lines'; episode: string }

export type AiTask = {
  id: TaskId
  /**
   * `null` while only its events have arrived: they may come before the
   * command that started it returns.
   */
  subject: AiTaskSubject | null
  done: number
  /** `null` until the first progress event. */
  total: number | null
  /** `null` while it runs. */
  outcome: TaskOutcome | null
}

export type AiTasksState = {
  tasks: AiTask[]
}

/**
 * The tasks started in this window, as their events tell. No command reads
 * them back: the events are all there is, so this keeps what they said, the
 * way the query cache keeps what commands returned. Only the latest task on
 * a subject is kept, ended or not, so a page can show how it went.
 */
export const useAiTasks = create<AiTasksState>()(() => ({ tasks: [] }))

function change(id: TaskId, update: (task: AiTask) => AiTask) {
  useAiTasks.setState(({ tasks }) => {
    const found = tasks.find((task) => task.id === id) ?? {
      id,
      subject: null,
      done: 0,
      total: null,
      outcome: null,
    }
    return { tasks: [...tasks.filter((task) => task.id !== id), update(found)] }
  })
}

function sameSubject(a: AiTaskSubject | null, b: AiTaskSubject) {
  return a !== null && a.kind === b.kind && a.episode === b.episode
}

/** The command that started a task has returned. */
export function aiTaskStarted(id: TaskId, subject: AiTaskSubject) {
  useAiTasks.setState(({ tasks }) => ({
    tasks: tasks.filter((task) => task.id === id || !sameSubject(task.subject, subject)),
  }))
  change(id, (task) => ({ ...task, subject }))
}

export function aiTaskProgressed({ task, done, total }: AiTaskProgress) {
  change(task, (found) => ({ ...found, done, total }))
}

export function aiTaskFinished({ task, outcome }: AiTaskFinished) {
  change(task, (found) => ({ ...found, outcome }))
}

/** The latest speaking of the episode's lines, running or ended; `undefined` if none was started. */
export function speakLinesTask(state: AiTasksState, episode: string): AiTask | undefined {
  return state.tasks.find((task) => sameSubject(task.subject, { kind: 'speak_lines', episode }))
}

export function useSpeakLinesTask(episode: string): AiTask | undefined {
  return useAiTasks((state) => speakLinesTask(state, episode))
}
