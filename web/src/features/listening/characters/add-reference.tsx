import { useState } from 'react'
import type { Language } from '@/bindings'
import { useListeningAddReference } from '@/backend/listening/use-listening-add-reference'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ReferenceField } from '@/features/listening/characters/reference-field'
import { emptyReference } from '@/features/listening/characters/reference-draft'
import { SeedField } from '@/features/listening/characters/seed-field'
import { parseSeed, randomSeed, voiceDesign } from '@/features/listening/characters/voice-design'

type Props = {
  name: string
  language: Language
  /** The character's prompt, to make the reference up from; `null` if it has none. */
  prompt: string | null
  /** The seed it starts from, the page's: the same for both languages. */
  initialSeed: number
}

/**
 * Fills a reference the character lacks, uploaded or made up from its
 * prompt. It cannot be replaced or removed afterwards, so it can be listened
 * to before saving. When both languages are missing, both start from the
 * same seed, so they sound like one person; when the other one is there,
 * the seed that made it was not kept, so this one is new. Either way it can
 * be typed or drawn again.
 */
export function AddReference({ name, language, prompt, initialSeed }: Props) {
  const [draft, setDraft] = useState(emptyReference)
  const [seed, setSeed] = useState(() => String(initialSeed))
  const seedValue = parseSeed(seed)
  const design = seedValue === null ? null : voiceDesign(prompt, seedValue)
  const add = useListeningAddReference()
  const file = draft.file

  return (
    <div className="flex flex-col gap-1.5">
      <ReferenceField
        language={language}
        draft={draft}
        design={design}
        disabled={add.isPending}
        seed={
          prompt !== null &&
          prompt.trim() !== '' && (
            <SeedField
              value={seed}
              disabled={add.isPending}
              onChange={setSeed}
              onRedraw={() => setSeed(String(randomSeed()))}
            />
          )
        }
        onChange={setDraft}
      />
      <div className="flex items-center gap-3">
        {file && (
          <Button
            size="sm"
            disabled={add.isPending}
            onClick={() => add.mutate({ name, language, audio: file.bytes, text: draft.text })}
          >
            保存
          </Button>
        )}
        <ErrorText error={add.error} />
      </div>
    </div>
  )
}
