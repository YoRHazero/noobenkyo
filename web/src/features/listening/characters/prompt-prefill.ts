import type { Gender } from '@/bindings'
import { ageText } from '@/features/listening/characters/age-text'

/** The age as the form holds it: an integer from 1 to 100, or `null` while it is not one. */
export function parseAge(text: string): number | null {
  if (!/^\d+$/.test(text.trim())) {
    return null
  }
  const age = Number(text.trim())
  return age >= 1 && age <= 100 ? age : null
}

/** The sentence a prompt starts with: 「十七岁的女性。」 */
export function promptPrefill(gender: Gender, age: number): string {
  return `${ageText(age)}岁的${gender === 'female' ? '女性' : '男性'}。`
}

/**
 * The prompt once gender or age changed: the old opening sentence is
 * replaced by the new one, and the rest is left as written. A prompt that no
 * longer starts with it is left alone.
 */
export function replacePrefill(prompt: string, from: string, to: string): string {
  return prompt.startsWith(from) ? to + prompt.slice(from.length) : prompt
}
