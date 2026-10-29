/**
 * What a reference is made up from: the character's prompt and a seed. The
 * seed is the form's own and is never stored: both languages of a new
 * character start from the same one, so they sound like one person.
 */
export type VoiceDesign = {
  prompt: string
  seed: number
}

/** `null` when there is no prompt to make a voice from. */
export function voiceDesign(prompt: string | null, seed: number): VoiceDesign | null {
  return prompt === null || prompt.trim() === '' ? null : { prompt, seed }
}

/** A fresh seed: a whole number the backend takes as 32 bits. */
export function randomSeed(): number {
  return crypto.getRandomValues(new Uint32Array(1))[0] ?? 0
}

/** A typed seed, if it is a whole number that fits in 32 bits; `null` otherwise. */
export function parseSeed(text: string): number | null {
  const trimmed = text.trim()
  if (!/^\d+$/.test(trimmed)) {
    return null
  }
  const seed = Number(trimmed)
  return seed < 2 ** 32 ? seed : null
}
