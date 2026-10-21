import { useState } from 'react'
import { Input } from '@/components/ui/input'

type Props = {
  disabled: boolean
  onSubmit: (text: string) => void
}

/** A Chinese answer; Enter submits once the input method has finished composing. */
export function ChineseInput({ disabled, onSubmit }: Props) {
  const [value, setValue] = useState('')
  return (
    <Input
      autoFocus
      value={value}
      disabled={disabled}
      placeholder="中文释义"
      className="h-11 text-lg md:text-lg"
      onChange={(event) => setValue(event.target.value)}
      onKeyDown={(event) => {
        if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
          onSubmit(value)
        }
      }}
    />
  )
}
