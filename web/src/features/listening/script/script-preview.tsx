import { useState, type ReactNode } from 'react'
import { useMutation } from '@tanstack/react-query'
import type { ScriptTarget } from '@/bindings'
import {
  useFetchListeningPreviewScript,
  useListeningPreviewScript,
} from '@/backend/listening/use-listening-preview-script'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ProblemList } from '@/features/listening/script/problem-list'
import { editedText } from '@/features/listening/script/script-editing'
import { deleteLine, scriptParts } from '@/features/listening/script/script-edits'
import { ScriptHeader } from '@/features/listening/script/script-header'
import { ScriptLines } from '@/features/listening/script/script-lines'
import type { ScriptEditor } from '@/features/listening/script/use-script-editor'

type Props = {
  target: ScriptTarget
  /** The text shown; every edit and saving start from it. The source holds it. */
  text: string
  /** The source has a newer text on its way than `text`, e.g. while typing. */
  stale?: boolean
  /** Nothing can be edited or saved, e.g. once the script has left the inbox. */
  locked?: boolean
  editor: ScriptEditor
  /**
   * Takes an edited text that passes the check: the source keeps it as its
   * text, or refuses it with problems (empty once kept).
   */
  edit: (text: string) => Promise<string[]>
  /** What saving means depends on where the script came from. */
  save: (text: string) => Promise<void>
  discard: () => Promise<void>
  /** What the source has to say above the script. */
  notice?: ReactNode
}

/**
 * Every script is looked at here before it is stored: its title, summary and
 * numbered lines, or the problems that stop it. A passing script can be
 * edited one place at a time; an edit is checked as a whole script first and
 * only a passing one reaches the source. Only the script on screen, with no
 * editor open, can be saved.
 */
export function ScriptPreview({
  target,
  text,
  stale = false,
  locked = false,
  editor,
  edit,
  save,
  discard,
  notice,
}: Props) {
  const preview = useListeningPreviewScript(text)
  const fetchPreview = useFetchListeningPreviewScript()
  const commit = useMutation({
    mutationFn: async (next: string) => {
      const checked = await fetchPreview(next)
      return checked.kind === 'failed' ? checked.problems : edit(next)
    },
  })
  const saving = useMutation({ mutationFn: save })
  const discarding = useMutation({ mutationFn: discard })
  /** Why the last delete did not go through. */
  const [deleteProblems, setDeleteProblems] = useState<string[]>([])
  const shown = preview.data
  const current = !preview.isPlaceholderData && !stale
  const passed = shown?.kind === 'passed'
  const busy = commit.isPending || saving.isPending || discarding.isPending
  const { editing, setEditing, status } = editor
  const canAct = passed && current && !locked && !busy && editing === null

  /** Hands the edited text over; the editor closes once the source keeps it. */
  const apply = async (next: string, onProblems: (problems: string[]) => void) => {
    setDeleteProblems([])
    const problems = await commit.mutateAsync(next).catch(() => null)
    if (problems === null) {
      return
    }
    if (problems.length > 0) {
      onProblems(problems)
    } else {
      setEditing(null)
    }
  }

  const confirm = () => {
    if (shown?.kind === 'passed' && editing) {
      void apply(editedText(scriptParts(shown), editing), (problems) =>
        setEditing({ ...editing, problems }),
      )
    }
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-3">
      {notice}
      <div className="min-h-0 flex-1 overflow-y-auto">
        {text.trim() === '' ? (
          <p className="text-sm text-muted-foreground">无内容</p>
        ) : !shown ? (
          <p className="text-sm text-muted-foreground">加载中…</p>
        ) : shown.kind === 'failed' ? (
          <ProblemList problems={shown.problems} />
        ) : (
          <div className="flex flex-col gap-2">
            <ScriptHeader
              title={shown.title}
              summary={shown.summary}
              titleUsed={target.kind === 'newEpisode'}
              editing={editing?.kind === 'header' ? editing : null}
              canEdit={canAct}
              busy={commit.isPending}
              canConfirm={status === 'changed'}
              onEdit={() => {
                const original = { title: shown.title, summary: shown.summary }
                setEditing({ kind: 'header', original, draft: original, problems: [] })
              }}
              onChange={(draft) => {
                if (editing?.kind === 'header') {
                  setEditing({ ...editing, draft })
                }
              }}
              onCancel={() => setEditing(null)}
              onConfirm={confirm}
            />
            <ScriptLines
              lines={shown.lines}
              editor={editor}
              canAct={canAct}
              busy={commit.isPending}
              onConfirm={confirm}
              onDelete={(index) =>
                void apply(deleteLine(scriptParts(shown), index), setDeleteProblems)
              }
            />
          </div>
        )}
      </div>
      <ProblemList problems={deleteProblems} />
      <div className="flex items-center justify-end gap-2">
        <ErrorText error={commit.error ?? saving.error ?? discarding.error} />
        <Button variant="outline" disabled={busy} onClick={() => discarding.mutate()}>
          丢弃
        </Button>
        <Button
          disabled={!passed || !current || locked || busy || editing !== null}
          onClick={() => saving.mutate(text)}
        >
          保存
        </Button>
      </div>
    </div>
  )
}
