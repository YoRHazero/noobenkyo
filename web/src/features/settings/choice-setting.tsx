import { ChoiceButton } from '@/components/choice-button'
import { SettingRow } from '@/features/settings/setting-row'

type Props<T extends string> = {
  label: string
  choices: { value: T; label: string }[]
  value: T
  fallback: T
  onChange: (value: T) => void
}

/** A setting that is one of a few choices. */
export function ChoiceSetting<T extends string>({
  label,
  choices,
  value,
  fallback,
  onChange,
}: Props<T>) {
  const name = (of: T) => choices.find((choice) => choice.value === of)?.label ?? of
  return (
    <SettingRow
      label={label}
      defaultText={name(fallback)}
      isDefault={value === fallback}
      onRestore={() => onChange(fallback)}
    >
      <div className="flex items-center gap-1 text-sm">
        {choices.map((choice) => (
          <ChoiceButton
            key={choice.value}
            chosen={value === choice.value}
            onClick={() => onChange(choice.value)}
          >
            {choice.label}
          </ChoiceButton>
        ))}
      </div>
    </SettingRow>
  )
}
