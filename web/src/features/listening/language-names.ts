import type { Language } from '@/bindings'

export const languageNames: Record<Language, string> = { ja: '日语', zh: '中文' }

/** Both languages, Japanese first, the order they are listed in everywhere. */
export const languages: readonly Language[] = ['ja', 'zh']
