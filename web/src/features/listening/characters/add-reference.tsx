import { useState } from 'react'
import type { Language } from '@/bindings'
import { useListeningAddReference } from '@/backend/listening/use-listening-add-reference'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ReferenceField } from '@/features/listening/characters/reference-field'
import { emptyReference } from '@/features/listening/characters/reference-draft'

type Props = {
  name: string
  language: Language
}

/**
 * Fills a reference the character lacks. It cannot be replaced or removed
 * afterwards, so the file can be listened to before saving.
 */
export function AddReference({ name, language }: Props) {
  const [draft, setDraft] = useState(emptyReference)
  const add = useListeningAddReference()
  const file = draft.file

  return (
    <div className="flex flex-col gap-1.5">
      <ReferenceField
        language={language}
        draft={draft}
        disabled={add.isPending}
        onChange={setDraft}
      />
      {file && (
        <div className="flex items-center gap-3">
          <Button
            size="sm"
            disabled={add.isPending}
            onClick={() => add.mutate({ name, language, audio: file.bytes, text: draft.text })}
          >
            保存
          </Button>
          <ErrorText error={add.error} />
        </div>
      )}
    </div>
  )
}
