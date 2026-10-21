import type { StudySettings } from '@/bindings'

/** The settings typed as numbers. */
export type NumberField =
  | 'maxStage'
  | 'capacity'
  | 'learnGap'
  | 'perStage'
  | 'base'
  | 'queueLimit'
  | 'reviewGap'
  | 'groupSize'
  | 'startSteps'

type Access = {
  read: (settings: StudySettings) => number | null
  write: (settings: StudySettings, value: number) => StudySettings
}

const quiz = (s: StudySettings, quiz: Partial<StudySettings['quiz']>) => ({
  ...s,
  quiz: { ...s.quiz, ...quiz },
})
const learn = (s: StudySettings, learn: Partial<StudySettings['learn']>) => ({
  ...s,
  learn: { ...s.learn, ...learn },
})
const review = (s: StudySettings, review: Partial<StudySettings['review']>) => ({
  ...s,
  review: { ...s.review, ...review },
})
const session = (s: StudySettings, options: Partial<StudySettings['review']['defaults']>) =>
  review(s, { defaults: { ...s.review.defaults, ...options } })

/** Where each number sits in the settings. */
export const numberFields: Record<NumberField, Access> = {
  maxStage: { read: (s) => s.quiz.max_stage, write: (s, n) => quiz(s, { max_stage: n }) },
  capacity: { read: (s) => s.learn.capacity, write: (s, n) => learn(s, { capacity: n }) },
  learnGap: { read: (s) => s.learn.recent_limit, write: (s, n) => learn(s, { recent_limit: n }) },
  perStage: {
    read: (s) => s.review.interval_per_stage,
    write: (s, n) => review(s, { interval_per_stage: n }),
  },
  base: { read: (s) => s.review.interval_base, write: (s, n) => review(s, { interval_base: n }) },
  queueLimit: {
    read: (s) => s.review.queue_limit,
    write: (s, n) => review(s, { queue_limit: n }),
  },
  reviewGap: {
    read: (s) => s.review.recent_limit,
    write: (s, n) => review(s, { recent_limit: n }),
  },
  // Never stored empty from here: an empty size shows as an empty box.
  groupSize: { read: (s) => s.review.defaults.size, write: (s, n) => session(s, { size: n }) },
  startSteps: {
    read: (s) => s.review.defaults.start_steps,
    write: (s, n) => session(s, { start_steps: n }),
  },
}

/**
 * A whole number as typed, or `null` for anything else. Whether the number
 * is allowed is the backend's to say.
 */
export function parseCount(text: string): number | null {
  const trimmed = text.trim()
  if (!/^\d+$/.test(trimmed)) {
    return null
  }
  const value = Number(trimmed)
  return value <= 0xffff_ffff ? value : null
}
