import { useState, type ChangeEvent, type KeyboardEvent } from 'react'
import { toKana } from 'wanakana'
import { Input } from '@/components/ui/input'
import { countKana, fitsKanaLimit } from '@/features/study/kana-limit'

type Props = {
  maxLength: number | null
  disabled?: boolean
  onSubmit: (kana: string) => void
}

/** A kana answer box: romaji turns into kana as you type; an IME works too. */
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

  const keyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
      onSubmit(toKana(value))
    }
  }

  return (
    <div className="flex items-center gap-3">
      <Input
        lang="ja"
        autoFocus
        value={value}
        disabled={disabled}
        placeholder="输入假名，可以直接打罗马字"
        className="text-lg"
        onChange={change}
        onKeyDown={keyDown}
      />
      {maxLength !== null && (
        <span className="shrink-0 text-sm text-muted-foreground tabular-nums">
          {countKana(value)} / {maxLength}
        </span>
      )}
    </div>
  )
}
