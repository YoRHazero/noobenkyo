import { describe, expect, it } from 'vitest'
import type { StudySettings } from '@/bindings'
import { parseCount } from '@/features/settings/number-fields'
import { fromDraft, sameSettings, toDraft, zoneChanges } from '@/features/settings/settings-draft'

const settings: StudySettings = {
  quiz: {
    max_stage: 7,
    kanji_order: ['Sentence', 'Translation', 'Reading', 'Comprehensive'],
    kana_order: ['Sentence', 'Translation', 'Comprehensive'],
    comprehensive: { glosses: false, writing: true, kana: true },
  },
  learn: { capacity: 10, order: 'Creation', recent_limit: 2 },
  review: {
    interval_per_stage: 3,
    interval_base: 1,
    queue_limit: 10,
    recent_limit: 2,
    defaults: { size: 30, start_steps: 1, order: 'Urgent' },
  },
}

describe('parseCount', () => {
  it('takes whole numbers only', () => {
    expect(parseCount(' 12 ')).toBe(12)
    expect(parseCount('')).toBeNull()
    expect(parseCount('1.5')).toBeNull()
    expect(parseCount('-1')).toBeNull()
  })
})

describe('the draft', () => {
  it('gives back the settings it was made from', () => {
    const back = fromDraft(toDraft(settings))
    expect(back && sameSettings(back, settings)).toBe(true)
  })

  it('writes each typed number to its place', () => {
    const draft = toDraft(settings)
    const next = fromDraft({
      ...draft,
      numbers: { ...draft.numbers, groupSize: '20', reviewGap: '3' },
    })
    expect(next?.review.defaults.size).toBe(20)
    expect(next?.review.recent_limit).toBe(3)
    expect(next?.learn.recent_limit).toBe(2)
  })

  it('has nothing to save while a number is not whole', () => {
    const draft = toDraft(settings)
    expect(fromDraft({ ...draft, numbers: { ...draft.numbers, capacity: '' } })).toBeNull()
  })
})

describe('zoneChanges', () => {
  it('names the capacity and the question orders that changed', () => {
    expect(zoneChanges(settings, settings)).toEqual([])
    const reordered = { ...settings.quiz, kana_order: [...settings.quiz.kana_order].reverse() }
    const next = { ...settings, learn: { ...settings.learn, capacity: 12 }, quiz: reordered }
    expect(zoneChanges(settings, next)).toEqual(['capacity', 'kanaOrder'])
    const longer = { ...settings, review: { ...settings.review, queue_limit: 12 } }
    expect(zoneChanges(settings, longer)).toEqual([])
  })
})
