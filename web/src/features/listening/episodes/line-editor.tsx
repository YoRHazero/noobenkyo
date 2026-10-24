import { useState } from 'react'
import type { LineView } from '@/bindings'
import { useListeningUpdateLine } from '@/backend/listening/use-listening-update-line'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { languageNames } from '@/features/listening/language-names'
import {
  changedLanguages,
  saveLabel,
  type LineDraft,
} from '@/features/listening/episodes/changed-languages'

type Props = {
  line: LineView
  onDone: () => void
}

/**
 * Edits a line's Japanese, reading marks and all, and its Chinese. Saving
 * sends each changed language on its own; when one is rejected, the ones
 * before it are already saved and the form stays with its problems.
 */
export function LineEditor({ line, onDone }: Props) {
  const [draft, setDraft] = useState<LineDraft>({ ja: line.ja, zh: line.zh })
  const [problems, setProblems] = useState<string[]>([])
  const update = useListeningUpdateLine()
  const changed = changedLanguages(line, draft)

  const save = async () => {
    setProblems([])
    for (const language of changed) {
      const rejected = await update
        .mutateAsync({ line: line.id, language, text: draft[language] })
        .catch(() => null)
      if (rejected === null) {
        return
      }
      if (rejected.length > 0) {
        setProblems(rejected.map((problem) => `${languageNames[language]}：${problem}`))
        return
      }
    }
    onDone()
  }

  return (
    <div className="flex flex-col gap-2">
      <Textarea
        lang="ja"
        aria-label="日语"
        spellCheck={false}
        value={draft.ja}
        disabled={update.isPending}
        className="min-h-12"
        onChange={(event) => setDraft({ ...draft, ja: event.target.value })}
      />
      <Textarea
        lang="zh"
        aria-label="中文"
        value={draft.zh}
        disabled={update.isPending}
        className="min-h-12"
        onChange={(event) => setDraft({ ...draft, zh: event.target.value })}
      />
      {problems.length > 0 && (
        <ul className="flex flex-col gap-1 text-sm text-destructive">
          {problems.map((problem) => (
            <li key={problem}>{problem}</li>
          ))}
        </ul>
      )}
      <div className="flex items-center justify-end gap-2">
        <ErrorText error={update.error} />
        <Button variant="outline" size="sm" disabled={update.isPending} onClick={onDone}>
          取消
        </Button>
        <Button size="sm" disabled={update.isPending || changed.length === 0} onClick={save}>
          {saveLabel(line, changed)}
        </Button>
      </div>
    </div>
  )
}
