import type { Language, LineView } from '@/bindings'
import { languageNames, languages } from '@/features/listening/language-names'

/** What the line editor holds: the Japanese as written, reading marks included, and the Chinese. */
export type LineDraft = Record<Language, string>

/** The languages whose text the draft changes; listening trims before it compares. */
export function changedLanguages(line: LineView, draft: LineDraft): Language[] {
  return languages.filter((language) => draft[language].trim() !== line[language])
}

/** The save button: it says which audio saving deletes. */
export function saveLabel(line: LineView, changed: readonly Language[]): string {
  const dropped = changed.filter((language) => line.audio.includes(language))
  return dropped.length === 0
    ? '保存'
    : `保存（删除${dropped.map((language) => languageNames[language]).join('、')}音频）`
}
