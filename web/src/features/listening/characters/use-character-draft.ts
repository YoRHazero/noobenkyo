import { useState } from 'react'
import { toKana } from 'wanakana'
import type { Gender, Language } from '@/bindings'
import type { CharacterInput } from '@/backend/listening/use-listening-create-character'
import { hasKanji } from '@/features/listening/characters/has-kanji'
import {
  parseAge,
  promptPrefill,
  replacePrefill,
} from '@/features/listening/characters/prompt-prefill'
import {
  emptyReference,
  type ReferenceDraft,
} from '@/features/listening/characters/reference-draft'

const startGender: Gender = 'female'
const startAge = 20

/**
 * The new-character form. A name without kanji is its own reading, so the
 * reading follows it. Changing gender or age rewrites the prompt's opening
 * sentence, as long as the prompt still starts with it.
 */
export function useCharacterDraft() {
  const [name, setName] = useState('')
  const [typedKana, setTypedKana] = useState('')
  const [gender, setGenderState] = useState(startGender)
  const [ageText, setAgeText] = useState(String(startAge))
  const [prefill, setPrefill] = useState(promptPrefill(startGender, startAge))
  const [prompt, setPrompt] = useState(prefill)
  const [references, setReferences] = useState<Record<Language, ReferenceDraft>>({
    ja: emptyReference,
    zh: emptyReference,
  })

  const kanaFollowsName = !hasKanji(name)
  const age = parseAge(ageText)

  const refill = (nextGender: Gender, nextAge: number | null) => {
    if (nextAge === null) {
      return
    }
    const next = promptPrefill(nextGender, nextAge)
    setPrompt(replacePrefill(prompt, prefill, next))
    setPrefill(next)
  }

  /** What to create, or `null` while the age is not a whole number from 1 to 100. */
  const input = (): CharacterInput | null =>
    age === null
      ? null
      : {
          name,
          kana: kanaFollowsName ? name.trim() : toKana(typedKana),
          gender,
          age,
          prompt: prompt.trim() === '' ? null : prompt,
          references: (['ja', 'zh'] as const).flatMap((language) => {
            const { file, text } = references[language]
            return file ? [{ language, audio: file.bytes, text }] : []
          }),
        }

  return {
    name,
    setName,
    kana: kanaFollowsName ? name.trim() : typedKana,
    kanaFollowsName,
    setKana: setTypedKana,
    gender,
    setGender: (next: Gender) => {
      setGenderState(next)
      refill(next, age)
    },
    ageText,
    ageValid: age !== null,
    setAgeText: (next: string) => {
      setAgeText(next)
      refill(gender, parseAge(next))
    },
    prompt,
    setPrompt,
    references,
    setReference: (language: Language, draft: ReferenceDraft) =>
      setReferences({ ...references, [language]: draft }),
    input,
  }
}
