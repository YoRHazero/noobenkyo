import type { ComprehensiveVisibility } from '@/bindings'
import { LabeledCheckbox } from '@/components/labeled-checkbox'
import { comprehensiveParts } from '@/features/settings/comprehensive-parts'

type Props = {
  value: ComprehensiveVisibility
  onChange: (value: ComprehensiveVisibility) => void
}

/** Which parts of a comprehensive question show before answering; the rest are covered. */
export function ComprehensiveField({ value, onChange }: Props) {
  return (
    <div className="flex items-center gap-4">
      {comprehensiveParts.map(({ part, label }) => (
        <LabeledCheckbox
          key={part}
          label={label}
          checked={value[part]}
          onChange={(checked) => onChange({ ...value, [part]: checked })}
        />
      ))}
    </div>
  )
}
