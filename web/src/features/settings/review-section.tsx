import type { PickOrder } from '@/bindings'
import { ChoiceSetting } from '@/features/settings/choice-setting'
import { NumberInput } from '@/features/settings/number-input'
import { parseCount } from '@/features/settings/number-fields'
import { NumberSetting } from '@/features/settings/number-setting'
import { SettingRow } from '@/features/settings/setting-row'
import type { DraftProps } from '@/features/settings/settings-draft'
import { SettingsSection } from '@/features/settings/settings-section'

const orders: { value: PickOrder; label: string }[] = [
  { value: 'Urgent', label: '权重顺序' },
  { value: 'Weighted', label: '加权随机' },
]

/** The review line: when senses are due, the queue, and how sessions start. */
export function ReviewSection(props: DraftProps) {
  const { draft, defaults, onDraft } = props
  const { review } = draft.settings
  const setText = (field: 'perStage' | 'base', text: string) =>
    onDraft({ ...draft, numbers: { ...draft.numbers, [field]: text } })
  const intervalIsDefault =
    parseCount(draft.numbers.perStage) === defaults.review.interval_per_stage &&
    parseCount(draft.numbers.base) === defaults.review.interval_base

  return (
    <SettingsSection title="复习">
      <SettingRow
        label="期望间隔"
        defaultText={`${defaults.review.interval_per_stage} 天/档 + ${defaults.review.interval_base} 天`}
        isDefault={intervalIsDefault}
        onRestore={() =>
          onDraft({
            ...draft,
            numbers: {
              ...draft.numbers,
              perStage: String(defaults.review.interval_per_stage),
              base: String(defaults.review.interval_base),
            },
          })
        }
      >
        <span className="flex items-center gap-2 text-sm">
          <NumberInput
            label="每档加几天"
            value={draft.numbers.perStage}
            unit="天/档"
            onChange={(text) => setText('perStage', text)}
          />
          <span className="text-muted-foreground">+</span>
          <NumberInput
            label="档位 0 的天数"
            value={draft.numbers.base}
            unit="天"
            onChange={(text) => setText('base', text)}
          />
        </span>
      </SettingRow>
      <NumberSetting field="queueLimit" label="队列上限" unit="个" {...props} />
      <NumberSetting field="reviewGap" label="最小间隔" unit="题" {...props} />
      <NumberSetting field="groupSize" label="复习容量" unit="个" {...props} />
      <NumberSetting field="startSteps" label="单词复习次数" unit="次" {...props} />
      <ChoiceSetting
        label="顺序"
        choices={orders}
        value={review.defaults.order}
        fallback={defaults.review.defaults.order}
        onChange={(order) =>
          onDraft({
            ...draft,
            settings: {
              ...draft.settings,
              review: { ...review, defaults: { ...review.defaults, order } },
            },
          })
        }
      />
    </SettingsSection>
  )
}
