import type { ReactNode } from 'react'
import { useMutation } from '@tanstack/react-query'
import { X } from 'lucide-react'
import type { Language } from '@/bindings'
import { useAiDesignVoice } from '@/backend/ai/use-ai-design-voice'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { openBinaryFile } from '@/files/open-binary-file'
import { ClipButton } from '@/features/listening/clip/clip-button'
import { wavFilters } from '@/features/listening/characters/wav-filters'
import type { ReferenceDraft } from '@/features/listening/characters/reference-draft'
import type { VoiceDesign } from '@/features/listening/characters/voice-design'

type Props = {
  language: Language
  draft: ReferenceDraft
  /** What a voice is made up from; `null` leaves only uploading. */
  design: VoiceDesign | null
  disabled?: boolean
  /** At the right end of the row with 上传 and 生成, such as the seed field. */
  seed?: ReactNode
  onChange: (draft: ReferenceDraft) => void
}

/** The name a made-up reference shows instead of a file's. */
const designedName = '生成的参考音'

/** What the text box says while empty, in its own language. */
const linePlaceholder: Record<Language, string> = { ja: 'セリフ', zh: '台词' }

/**
 * One reference to add: upload a WAV or make one up from the prompt, listen
 * to it, and write what is said in it. A made-up one says the text written
 * first.
 */
export function ReferenceField({ language, draft, design, disabled, seed, onChange }: Props) {
  const choose = useMutation({
    mutationFn: () => openBinaryFile(wavFilters),
    onSuccess: (file) => {
      if (file !== null) {
        onChange({ ...draft, file })
      }
    },
  })
  const designing = useAiDesignVoice()
  const busy = disabled || choose.isPending || designing.isPending
  const file = draft.file

  const text = (
    <Textarea
      lang={language}
      aria-label="台词"
      placeholder={linePlaceholder[language]}
      value={draft.text}
      disabled={busy}
      className="h-20 resize-none overflow-y-auto field-sizing-fixed"
      onChange={(event) => onChange({ ...draft, text: event.target.value })}
    />
  )

  if (file === null) {
    return (
      <div className="flex flex-col gap-1.5">
        {text}
        <div className="flex flex-wrap items-center gap-2">
          <Button variant="outline" size="sm" disabled={busy} onClick={() => choose.mutate()}>
            上传
          </Button>
          <Button
            variant="outline"
            size="sm"
            disabled={busy || design === null || draft.text.trim() === ''}
            onClick={() =>
              design &&
              designing.mutate(
                { prompt: design.prompt, language, text: draft.text, seed: design.seed },
                {
                  onSuccess: (bytes) => onChange({ ...draft, file: { name: designedName, bytes } }),
                },
              )
            }
          >
            {designing.isPending ? '生成中…' : '生成'}
          </Button>
          <ErrorText error={choose.error ?? designing.error} />
          {seed && <div className="ml-auto">{seed}</div>}
        </div>
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
      {text}
    </div>
  )
}
