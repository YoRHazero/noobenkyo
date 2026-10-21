import type { ImportOptions } from '@/bindings'
import { LabeledCheckbox } from '@/components/labeled-checkbox'

type Props = {
  options: ImportOptions
  disabled: boolean
  onChange: (options: ImportOptions) => void
}

/** The choices for this one import; nothing remembers them. */
export function ImportOptionsFields({ options, disabled, onChange }: Props) {
  const set = (key: keyof ImportOptions) => (value: boolean) =>
    onChange({ ...options, [key]: value })
  return (
    <div className="flex flex-wrap gap-6">
      <LabeledCheckbox
        label="去掉纯片假名词"
        checked={options.drop_pure_katakana}
        disabled={disabled}
        onChange={set('drop_pure_katakana')}
      />
      <LabeledCheckbox
        label="识别活用形"
        checked={options.detect_inflections}
        disabled={disabled}
        onChange={set('detect_inflections')}
      />
      <LabeledCheckbox
        label="保留进度"
        checked={options.keep_progress}
        disabled={disabled}
        onChange={set('keep_progress')}
      />
    </div>
  )
}
