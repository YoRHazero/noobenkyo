import type { LearnOrder } from '@/bindings'
import { ChoiceSetting } from '@/features/settings/choice-setting'
import { NumberSetting } from '@/features/settings/number-setting'
import type { DraftProps } from '@/features/settings/settings-draft'
import { SettingsSection } from '@/features/settings/settings-section'

const orders: { value: LearnOrder; label: string }[] = [
  { value: 'Creation', label: '顺序' },
  { value: 'Random', label: '随机' },
]

/** The learning line: its zone and the order senses enter it. */
export function LearnSection(props: DraftProps) {
  const { draft, defaults, onDraft } = props
  const learn = draft.settings.learn
  return (
    <SettingsSection title="学习">
      <NumberSetting field="capacity" label="学习区容量" unit="个" {...props} />
      <ChoiceSetting
        label="进区顺序"
        choices={orders}
        value={learn.order}
        fallback={defaults.learn.order}
        onChange={(order) =>
          onDraft({ ...draft, settings: { ...draft.settings, learn: { ...learn, order } } })
        }
      />
      <NumberSetting field="learnGap" label="最小间隔" unit="题" {...props} />
    </SettingsSection>
  )
}
