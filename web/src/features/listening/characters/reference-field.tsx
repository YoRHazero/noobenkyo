import { useMutation } from '@tanstack/react-query'
import { X } from 'lucide-react'
import type { Language } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { openBinaryFile } from '@/files/open-binary-file'
import { ClipButton } from '@/features/listening/clip/clip-button'
import { wavFilters } from '@/features/listening/characters/wav-filters'
import type { ReferenceDraft } from '@/features/listening/characters/reference-draft'

type Props = {
  language: Language
  draft: ReferenceDraft
  disabled?: boolean
  onChange: (draft: ReferenceDraft) => void
}

/**
 * One reference to add: upload a WAV, listen to it, write what is said in it.
 * Generating one is not there yet, so its button is only reserved.
 */
export function ReferenceField({ language, draft, disabled, onChange }: Props) {
  const choose = useMutation({
    mutationFn: () => openBinaryFile(wavFilters),
    onSuccess: (file) => {
      if (file !== null) {
        onChange({ ...draft, file })
      }
    },
  })
  const file = draft.file

  if (file === null) {
    return (
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          disabled={disabled || choose.isPending}
          onClick={() => choose.mutate()}
        >
          上传…
        </Button>
        <Button variant="outline" size="sm" disabled>
          生成…
        </Button>
        <ErrorText error={choose.error} />
      </div>
    )
  }
  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex items-center gap-1 text-sm">
        <span className="truncate">{file.name}</span>
        <ClipButton
          clipKey={`upload:${language}:${file.name}`}
          label="试听"
          load={() =>
            Promise.resolve(new Blob([new Uint8Array(file.bytes)], { type: 'audio/wav' }))
          }
        />
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="去掉"
          disabled={disabled}
          onClick={() => onChange({ ...draft, file: null })}
        >
          <X />
        </Button>
      </div>
      <Input
        lang={language}
        aria-label="录音里说的话"
        placeholder="录音里说的话"
        value={draft.text}
        disabled={disabled}
        className="h-8"
        onChange={(event) => onChange({ ...draft, text: event.target.value })}
      />
    </div>
  )
}
