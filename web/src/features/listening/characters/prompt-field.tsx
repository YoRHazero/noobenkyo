import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { voicePresets } from '@/features/listening/characters/voice-presets'

type Props = {
  value: string
  disabled?: boolean
  onChange: (value: string) => void
}

/** The prompt, and the presets that append a voice to it. Left empty, there is none. */
export function PromptField({ value, disabled, onChange }: Props) {
  return (
    <div className="flex flex-col gap-1.5">
      <Textarea
        aria-label="prompt"
        value={value}
        disabled={disabled}
        className="h-32 resize-none overflow-y-auto field-sizing-fixed"
        onChange={(event) => onChange(event.target.value)}
      />
      <div className="flex flex-wrap items-center gap-1">
        <span className="mr-1 text-xs text-muted-foreground">预设</span>
        {voicePresets.map((preset) => (
          <Button
            key={preset.name}
            variant="outline"
            size="sm"
            className="h-6 px-2 text-xs"
            disabled={disabled}
            onClick={() => onChange(value + preset.description)}
          >
            {preset.name}
          </Button>
        ))}
      </div>
    </div>
  )
}
