import { useId } from 'react'
import { RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { parseSeed } from '@/features/listening/characters/voice-design'

type Props = {
  /** As typed; marked red unless it is a whole number that fits in 32 bits. */
  value: string
  disabled?: boolean
  onChange: (value: string) => void
  /** Draws a new seed for this language only. */
  onRedraw: () => void
}

/** The seed one language's voice is made up with, typed or drawn again. */
export function SeedField({ value, disabled, onChange, onRedraw }: Props) {
  const id = useId()
  return (
    <div className="flex items-center gap-1.5">
      <label htmlFor={id} className="text-xs text-muted-foreground">
        seed:
      </label>
      <Input
        id={id}
        inputMode="numeric"
        value={value}
        aria-invalid={parseSeed(value) === null}
        disabled={disabled}
        className="h-7 w-28 text-xs tabular-nums"
        onChange={(event) => onChange(event.target.value)}
      />
      <Button
        variant="ghost"
        size="icon-sm"
        className="size-7"
        aria-label="重新抽取 seed"
        disabled={disabled}
        onClick={onRedraw}
      >
        <RefreshCw />
      </Button>
    </div>
  )
}
