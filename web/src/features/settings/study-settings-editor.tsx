import { useState } from 'react'
import type { LearningOrder, StudySettings } from '@/bindings'
import { LabeledCheckbox } from '@/components/labeled-checkbox'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { learningOrderLabel } from '@/features/settings/labels'
import { NumberField } from '@/features/settings/number-field'
import { NumberListField } from '@/features/settings/number-list-field'
import { QuestionOrderEditor } from '@/features/settings/question-order-editor'

const learningOrders: LearningOrder[] = ['Creation', 'Random']

type Props = {
  initial: StudySettings
  pending: boolean
  onSave: (settings: StudySettings) => void
  onReset: () => void
}

/** Edits a draft of the study settings; saving replaces them as a whole. */
export function StudySettingsEditor({ initial, pending, onSave, onReset }: Props) {
  const [draft, setDraft] = useState(initial)
  const { quiz, workspace } = draft
  const setQuiz = (patch: Partial<StudySettings['quiz']>) =>
    setDraft({ ...draft, quiz: { ...quiz, ...patch } })
  const setWorkspace = (patch: Partial<StudySettings['workspace']>) =>
    setDraft({ ...draft, workspace: { ...workspace, ...patch } })
  const setVisible = (patch: Partial<StudySettings['quiz']['comprehensive']>) =>
    setQuiz({ comprehensive: { ...quiz.comprehensive, ...patch } })

  return (
    <div className="flex flex-col gap-6">
      <fieldset className="flex flex-col gap-4">
        <legend className="mb-2 font-medium">题型</legend>
        <QuestionOrderEditor
          label="有汉字的词"
          value={quiz.kanji_order}
          onChange={(kanji_order) => setQuiz({ kanji_order })}
        />
        <QuestionOrderEditor
          label="纯假名的词"
          value={quiz.kana_order}
          onChange={(kana_order) => setQuiz({ kana_order })}
        />
        <div className="flex flex-col gap-2">
          <Label>综合题答题前显示</Label>
          <div className="flex gap-6">
            <LabeledCheckbox
              label="汉字"
              checked={quiz.comprehensive.writing}
              onChange={(writing) => setVisible({ writing })}
            />
            <LabeledCheckbox
              label="读音"
              checked={quiz.comprehensive.kana}
              onChange={(kana) => setVisible({ kana })}
            />
            <LabeledCheckbox
              label="释义"
              checked={quiz.comprehensive.glosses}
              onChange={(glosses) => setVisible({ glosses })}
            />
          </div>
        </div>
      </fieldset>

      <fieldset className="grid grid-cols-3 gap-4">
        <legend className="mb-2 font-medium">工作区</legend>
        <NumberField
          label="学习区容量"
          value={workspace.learning_capacity}
          onChange={(learning_capacity) => setWorkspace({ learning_capacity })}
        />
        <NumberField
          label="复习区容量（无限模式）"
          value={workspace.review_capacity}
          onChange={(review_capacity) => setWorkspace({ review_capacity })}
        />
        <NumberField
          label="每组数量（定组模式）"
          value={workspace.fixed_group_size}
          onChange={(fixed_group_size) => setWorkspace({ fixed_group_size })}
        />
        <NumberField
          label="每天新词上限"
          value={workspace.daily_new_limit}
          onChange={(daily_new_limit) => setWorkspace({ daily_new_limit })}
        />
        <NumberField
          label="最近几题不重复（k 的上限）"
          value={workspace.recent_limit}
          onChange={(recent_limit) => setWorkspace({ recent_limit })}
        />
        <div className="flex flex-col gap-1.5">
          <Label>新词进区顺序</Label>
          <div className="flex gap-2">
            {learningOrders.map((order) => (
              <Button
                key={order}
                size="sm"
                variant={workspace.learning_order === order ? 'secondary' : 'outline'}
                onClick={() => setWorkspace({ learning_order: order })}
              >
                {learningOrderLabel[order]}
              </Button>
            ))}
          </div>
        </div>
      </fieldset>

      <fieldset className="grid grid-cols-2 gap-4">
        <legend className="mb-2 font-medium">复习权重</legend>
        <NumberListField
          label="各档位的期望间隔（天，档位 0～N）"
          value={workspace.stage_intervals}
          onChange={(stage_intervals) => setWorkspace({ stage_intervals })}
        />
        <NumberListField
          label="定组时各档位的比例"
          value={workspace.group_stage_ratios}
          onChange={(group_stage_ratios) => setWorkspace({ group_stage_ratios })}
        />
        <NumberField
          label="到期时的权重（%）"
          value={workspace.due_weight_percent}
          onChange={(due_weight_percent) => setWorkspace({ due_weight_percent })}
        />
        <NumberField
          label="每次正确对间隔的加成（%）"
          value={workspace.pass_bonus_percent}
          onChange={(pass_bonus_percent) => setWorkspace({ pass_bonus_percent })}
        />
      </fieldset>

      <div className="flex items-center gap-3">
        <Button disabled={pending} onClick={() => onSave(draft)}>
          保存
        </Button>
        <Button variant="outline" disabled={pending} onClick={onReset}>
          恢复默认
        </Button>
      </div>
    </div>
  )
}
