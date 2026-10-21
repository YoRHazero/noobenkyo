import type { StudySettings } from '@/bindings'
import { numberFields, parseCount, type NumberField } from '@/features/settings/number-fields'

/** The settings being edited: the numbers as typed, everything else as chosen. */
export type SettingsDraft = {
  settings: StudySettings
  numbers: Record<NumberField, string>
}

/** What each part of the form edits, and the defaults it offers to restore. */
export type DraftProps = {
  draft: SettingsDraft
  defaults: StudySettings
  onDraft: (draft: SettingsDraft) => void
}

const fields = Object.keys(numberFields) as NumberField[]

export function toDraft(settings: StudySettings): SettingsDraft {
  const numbers = Object.fromEntries(
    fields.map((field) => [field, String(numberFields[field].read(settings) ?? '')]),
  ) as Record<NumberField, string>
  return { settings, numbers }
}

/** The settings to save; `null` while a number is not a whole number. */
export function fromDraft(draft: SettingsDraft): StudySettings | null {
  let settings = draft.settings
  for (const field of fields) {
    const value = parseCount(draft.numbers[field])
    if (value === null) {
      return null
    }
    settings = numberFields[field].write(settings, value)
  }
  return settings
}

export function sameSettings(a: StudySettings, b: StudySettings): boolean {
  return JSON.stringify(a) === JSON.stringify(b)
}

/** The settings whose change clears the learning zone. */
export type ZoneSetting = 'capacity' | 'kanjiOrder' | 'kanaOrder'

/** What in `next` clears the learning zone when saved; empty when nothing does. */
export function zoneChanges(saved: StudySettings, next: StudySettings): ZoneSetting[] {
  const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b)
  const changes: ZoneSetting[] = []
  if (saved.learn.capacity !== next.learn.capacity) {
    changes.push('capacity')
  }
  if (!same(saved.quiz.kanji_order, next.quiz.kanji_order)) {
    changes.push('kanjiOrder')
  }
  if (!same(saved.quiz.kana_order, next.quiz.kana_order)) {
    changes.push('kanaOrder')
  }
  return changes
}
