import type { ComprehensiveVisibility } from '@/bindings'

/** The parts of a comprehensive question, as the settings name them. */
export const comprehensiveParts: { part: keyof ComprehensiveVisibility; label: string }[] = [
  { part: 'glosses', label: '释义' },
  { part: 'writing', label: '汉字' },
  { part: 'kana', label: '读音' },
]
