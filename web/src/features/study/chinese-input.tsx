import { useState, type KeyboardEvent } from 'react'
import { Input } from '@/components/ui/input'

type Props = {
  disabled?: boolean
  onSubmit: (text: string) => void
}

/** A Chinese answer box; Enter submits once the IME has finished composing. */
export function ChineseInput({ disabled, onSubmit }: Props) {
  const [value, setValue] = useState('')

  const keyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
      onSubmit(value.trim())
    }
  }

  return (
    <Input
      autoFocus
      value={value}
      disabled={disabled}
      placeholder="输入目标词的中文意思"
      className="text-lg"
      onChange={(event) => setValue(event.target.value)}
      onKeyDown={keyDown}
    />
  )
}
