import type { QuestionType, StudySettings } from '@/bindings'
import { ComprehensiveField } from '@/features/settings/comprehensive-field'
import { comprehensiveParts } from '@/features/settings/comprehensive-parts'
import { NumberSetting } from '@/features/settings/number-setting'
import { QuestionOrderEditor } from '@/features/settings/question-order-editor'
import { questionTypeName } from '@/features/settings/question-type-names'
import { SettingRow } from '@/features/settings/setting-row'
import type { DraftProps } from '@/features/settings/settings-draft'
import { SettingsSection } from '@/features/settings/settings-section'

const kanjiTypes: QuestionType[] = ['Sentence', 'Translation', 'Reading', 'Comprehensive']
/** A word without kanji has nothing to read, so it is never offered a reading question. */
const kanaTypes: QuestionType[] = ['Sentence', 'Translation', 'Comprehensive']

const orderText = (order: QuestionType[]) => order.map((type) => questionTypeName[type]).join('、')
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b)

/** What both lines share: the top stage, the question orders, the comprehensive question. */
export function QuizSection(props: DraftProps) {
  const { draft, defaults, onDraft } = props
  const quiz = draft.settings.quiz
  const set = (change: Partial<StudySettings['quiz']>) =>
    onDraft({ ...draft, settings: { ...draft.settings, quiz: { ...quiz, ...change } } })
  const shown = comprehensiveParts
    .filter(({ part }) => defaults.quiz.comprehensive[part])
    .map(({ label }) => label)

  return (
    <SettingsSection title="题目">
      <NumberSetting field="maxStage" label="最高档位" {...props} />
      <SettingRow
        label="题型顺序（有汉字）"
        defaultText={orderText(defaults.quiz.kanji_order)}
        isDefault={same(quiz.kanji_order, defaults.quiz.kanji_order)}
        onRestore={() => set({ kanji_order: defaults.quiz.kanji_order })}
      >
        <QuestionOrderEditor
          value={quiz.kanji_order}
          types={kanjiTypes}
          onChange={(kanji_order) => set({ kanji_order })}
        />
      </SettingRow>
      <SettingRow
        label="题型顺序（纯假名）"
        defaultText={orderText(defaults.quiz.kana_order)}
        isDefault={same(quiz.kana_order, defaults.quiz.kana_order)}
        onRestore={() => set({ kana_order: defaults.quiz.kana_order })}
      >
        <QuestionOrderEditor
          value={quiz.kana_order}
          types={kanaTypes}
          onChange={(kana_order) => set({ kana_order })}
        />
      </SettingRow>
      <SettingRow
        label="综合题作答前显示"
        defaultText={shown.length > 0 ? shown.join('、') : '无'}
        isDefault={same(quiz.comprehensive, defaults.quiz.comprehensive)}
        onRestore={() => set({ comprehensive: defaults.quiz.comprehensive })}
      >
        <ComprehensiveField
          value={quiz.comprehensive}
          onChange={(comprehensive) => set({ comprehensive })}
        />
      </SettingRow>
    </SettingsSection>
  )
}
