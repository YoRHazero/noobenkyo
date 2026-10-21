import { NumberInput } from '@/features/settings/number-input'
import { numberFields, parseCount, type NumberField } from '@/features/settings/number-fields'
import { SettingRow } from '@/features/settings/setting-row'
import type { DraftProps } from '@/features/settings/settings-draft'

type Props = DraftProps & {
  field: NumberField
  label: string
  unit?: string
}

/** A setting that is one number. */
export function NumberSetting({ field, label, unit, draft, defaults, onDraft }: Props) {
  const fallback = numberFields[field].read(defaults)
  const set = (text: string) => onDraft({ ...draft, numbers: { ...draft.numbers, [field]: text } })
  return (
    <SettingRow
      label={label}
      defaultText={unit ? `${fallback} ${unit}` : String(fallback)}
      isDefault={parseCount(draft.numbers[field]) === fallback}
      onRestore={() => set(String(fallback ?? ''))}
    >
      <NumberInput label={label} value={draft.numbers[field]} unit={unit} onChange={set} />
    </SettingRow>
  )
}
