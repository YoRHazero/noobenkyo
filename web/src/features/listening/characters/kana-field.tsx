import type { ChangeEvent } from 'react'
import { toKana } from 'wanakana'
import { Input } from '@/components/ui/input'

type Props = {
  value: string
  disabled?: boolean
  onChange: (value: string) => void
}

/**
 * A reading: romaji turns into kana as it is typed, lowercase into hiragana
 * and uppercase into katakana (ARISU → アリス); a Japanese input method works
 * too, and is left alone while composing.
 */
export function KanaField({ value, disabled, onChange }: Props) {
  const change = (event: ChangeEvent<HTMLInputElement>) => {
    const raw = event.target.value
    const composing = (event.nativeEvent as InputEvent).isComposing
    onChange(composing ? raw : toKana(raw, { IMEMode: true }))
  }
  return (
    <Input
      lang="ja"
      aria-label="读音"
      value={value}
      disabled={disabled}
      className="h-8"
      onChange={change}
    />
  )
}
