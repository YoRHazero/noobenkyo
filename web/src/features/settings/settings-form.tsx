import { useState } from 'react'
import type { StudySettings } from '@/bindings'
import { useUpdateStudySettings } from '@/backend/study/use-update-study-settings'
import { LearnSection } from '@/features/settings/learn-section'
import { QuizSection } from '@/features/settings/quiz-section'
import { ReviewSection } from '@/features/settings/review-section'
import { SaveBar } from '@/features/settings/save-bar'
import { fromDraft, sameSettings, toDraft, zoneChanges } from '@/features/settings/settings-draft'

type Props = {
  saved: StudySettings
  defaults: StudySettings
}

/**
 * Every setting, saved together as one replacement. Restoring defaults only
 * changes the form; nothing is saved until 保存. A refused save keeps the form.
 */
export function SettingsForm({ saved, defaults }: Props) {
  const [draft, setDraft] = useState(() => toDraft(saved))
  const update = useUpdateStudySettings()
  const next = fromDraft(draft)
  const parts = { draft, defaults, onDraft: setDraft }

  return (
    <div className="flex flex-col gap-6">
      <QuizSection {...parts} />
      <LearnSection {...parts} />
      <ReviewSection {...parts} />
      <SaveBar
        changed={next === null || !sameSettings(next, saved)}
        complete={next !== null}
        atDefaults={next !== null && sameSettings(next, defaults)}
        zoneChanges={next ? zoneChanges(saved, next) : []}
        busy={update.isPending}
        error={update.error}
        onDefaults={() => setDraft(toDraft(defaults))}
        onRevert={() => setDraft(toDraft(saved))}
        onSave={() => next && update.mutate(next)}
      />
    </div>
  )
}
