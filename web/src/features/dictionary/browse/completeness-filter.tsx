import type { Completeness } from '@/bindings'
import { ChoiceButton } from '@/features/dictionary/browse/choice-button'

const layers: { layer: Completeness; label: string; onlyWithoutProgress: boolean }[] = [
  { layer: 'WordOnly', label: '只有写法', onlyWithoutProgress: true },
  { layer: 'ReadingOnly', label: '到读音', onlyWithoutProgress: true },
  { layer: 'NoSentence', label: '没例句', onlyWithoutProgress: false },
  { layer: 'WithSentence', label: '有例句', onlyWithoutProgress: false },
]

type Props = {
  value: Completeness[]
  /** A progress condition only matches senses, so the upper layers are off. */
  progressChosen: boolean
  onChange: (value: Completeness[]) => void
}

/** How far down a row is written; several at once, none for any. */
export function CompletenessFilter({ value, progressChosen, onChange }: Props) {
  const toggle = (layer: Completeness) =>
    onChange(value.includes(layer) ? value.filter((l) => l !== layer) : [...value, layer])
  return (
    <div className="flex items-center gap-1 text-sm">
      <span className="text-muted-foreground">层级：</span>
      {layers.map(({ layer, label, onlyWithoutProgress }) => (
        <ChoiceButton
          key={layer}
          chosen={value.includes(layer)}
          disabled={onlyWithoutProgress && progressChosen}
          onClick={() => toggle(layer)}
        >
          {label}
        </ChoiceButton>
      ))}
    </div>
  )
}
