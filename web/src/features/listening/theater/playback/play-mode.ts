import type { Language } from '@/bindings'

/** How a voice's lines are played: one language, or both in either order. */
export type PlayMode = 'ja' | 'zh' | 'ja-zh' | 'zh-ja'

export const playModes: readonly PlayMode[] = ['ja', 'zh', 'ja-zh', 'zh-ja']

export const playModeNames: Record<PlayMode, string> = {
  ja: '只日语',
  zh: '只中文',
  'ja-zh': '先日后中',
  'zh-ja': '先中后日',
}

/** The languages a mode plays, in order. */
export function modeLanguages(mode: PlayMode): Language[] {
  switch (mode) {
    case 'ja':
      return ['ja']
    case 'zh':
      return ['zh']
    case 'ja-zh':
      return ['ja', 'zh']
    case 'zh-ja':
      return ['zh', 'ja']
  }
}

/** Whether a voice with references in `languages` can use the mode. */
export function modeAllowed(mode: PlayMode, languages: readonly Language[]): boolean {
  return modeLanguages(mode).every((language) => languages.includes(language))
}

/** Where a voice starts: both languages, Japanese first, when it has both. */
export function startMode(languages: readonly Language[]): PlayMode {
  if (languages.includes('ja') && !languages.includes('zh')) {
    return 'ja'
  }
  if (languages.includes('zh') && !languages.includes('ja')) {
    return 'zh'
  }
  return 'ja-zh'
}
