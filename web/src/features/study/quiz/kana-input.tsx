import { useState, type ChangeEvent } from 'react'
import { toKana } from 'wanakana'
import { Input } from '@/components/ui/input'
import { countKana, fitsKanaLimit } from '@/features/study/quiz/kana-limit'

type Props = {
  /** `null`: no limit. */
  maxLength: number | null
  disabled: boolean
  onSubmit: (kana: string) => void
}

/**
 * A kana answer: romaji turns into kana as it is typed, and a Japanese input
 * method works too. While an input method is composing, nothing is converted
 * or limited. Submitting turns a trailing n into ん.
 */
export function KanaInput({ maxLength, disabled, onSubmit }: Props) {
  const [value, setValue] = useState('')

  const change = (event: ChangeEvent<HTMLInputElement>) => {
    const raw = event.target.value
    const composing = (event.nativeEvent as InputEvent).isComposing
    const next = composing ? raw : toKana(raw, { IMEMode: true })
    if (composing || fitsKanaLimit(next, maxLength)) {
      setValue(next)
    }
  }

  return (
    <div className="flex items-center gap-3">
      <Input
        lang="ja"
        autoFocus
        value={value}
        disabled={disabled}
        placeholder="词典形假名（可输入罗马字）"
        className="h-11 text-lg md:text-lg"
        onChange={change}
        onKeyDown={(event) => {
          if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
            onSubmit(toKana(value))
          }
        }}
      />
      {maxLength !== null && (
        <span className="shrink-0 text-sm text-muted-foreground tabular-nums">
          {countKana(value)} / {maxLength}
        </span>
      )}
    </div>
  )
}
