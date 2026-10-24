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

/**
 * Where every voice starts. Modes are how the user wants to listen, not what
 * a character's references allow: whether an episode can play depends only
 * on the audio its lines have.
 */
export const startMode: PlayMode = 'ja-zh'
