import type { Language } from '@/bindings'
import { loadListeningReference } from '@/backend/listening/load-listening-reference'
import { ClipButton } from '@/features/listening/clip/clip-button'
import { AddReference } from '@/features/listening/characters/add-reference'

type Props = {
  name: string
  language: Language
  /** What is said in the reference, or `null` when the character has none. */
  text: string | null
  /** The character's prompt; `null` if it has none. */
  prompt: string | null
  /** Where a missing reference's seed starts: the same for both languages. */
  seed: number
}

/** A reference to listen to, or the way to add the missing one. */
export function ReferenceRow({ name, language, text, prompt, seed }: Props) {
  if (text === null) {
    return <AddReference name={name} language={language} prompt={prompt} initialSeed={seed} />
  }
  return (
    // Lined up with the label and the text's first line, however many lines it takes.
    <div className="flex items-start gap-1">
      <ClipButton
        clipKey={`reference:${name}:${language}`}
        label="试听"
        load={() => loadListeningReference(name, language)}
        className="-my-0.5 size-6"
      />
      <span lang={language} className="leading-5">
        「{text}」
      </span>
    </div>
  )
}
