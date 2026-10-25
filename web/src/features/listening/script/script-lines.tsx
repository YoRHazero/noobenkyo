import { Fragment } from 'react'
import { Pencil, Plus, Trash2 } from 'lucide-react'
import type { PreviewLine } from '@/bindings'
import { ConfirmButton } from '@/components/confirm-button'
import { Button } from '@/components/ui/button'
import { openInsert } from '@/features/listening/script/script-editing'
import { ScriptLineEditor } from '@/features/listening/script/script-line-editor'
import { ScriptLineView } from '@/features/listening/script/script-line-view'
import type { ScriptEditor } from '@/features/listening/script/use-script-editor'

type Props = {
  lines: PreviewLine[]
  editor: ScriptEditor
  /** Whether editing, adding and deleting can start: no editor is open and the preview is current. */
  canAct: boolean
  /** A confirm or a delete is on its way. */
  busy: boolean
  onConfirm: () => void
  onDelete: (index: number) => void
}

/**
 * A script's lines, numbered from 1 as the checks number them. Each can be
 * edited or deleted, and a line can be added first or after any line; the
 * order of the lines cannot change.
 */
export function ScriptLines({ lines, editor, canAct, busy, onConfirm, onDelete }: Props) {
  const { editing, setEditing, status } = editor
  const speakers = [...new Set(['旁白', ...lines.map((line) => line.speaker)])]
  const lineEditor = (number: number | null) =>
    editing &&
    editing.kind !== 'header' && (
      <ScriptLineEditor
        key="editor"
        number={number}
        draft={editing.draft}
        problems={editing.problems}
        speakers={speakers}
        busy={busy}
        canConfirm={status === 'changed'}
        onChange={(draft) => setEditing({ ...editing, draft })}
        onCancel={() => setEditing(null)}
        onConfirm={onConfirm}
      />
    )
  const insertingAt = editing?.kind === 'insert' ? editing.at : null

  return (
    <div className="flex flex-col">
      <Button
        variant="ghost"
        size="xs"
        className="self-start text-muted-foreground"
        disabled={!canAct}
        onClick={() => setEditing(openInsert(0))}
      >
        <Plus />
        加在开头
      </Button>
      <ol className="flex flex-col divide-y text-sm">
        {lines.map((line, index) => (
          <Fragment key={index}>
            {insertingAt === index && lineEditor(null)}
            {editing?.kind === 'line' && editing.index === index ? (
              lineEditor(index + 1)
            ) : (
              <ScriptLineView
                number={index + 1}
                line={line}
                actions={
                  <>
                    <Button
                      variant="ghost"
                      size="icon-sm"
                      aria-label="编辑"
                      disabled={!canAct}
                      onClick={() => {
                        const original = { speaker: line.speaker, ja: line.ja, zh: line.zh }
                        setEditing({ kind: 'line', index, original, draft: original, problems: [] })
                      }}
                    >
                      <Pencil />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-sm"
                      aria-label="在下面加一行"
                      disabled={!canAct}
                      onClick={() => setEditing(openInsert(index + 1))}
                    >
                      <Plus />
                    </Button>
                    <ConfirmButton
                      label={<Trash2 />}
                      ariaLabel="删除"
                      confirmLabel="确认删除"
                      size="icon-sm"
                      disabled={!canAct}
                      onConfirm={() => onDelete(index)}
                    />
                  </>
                }
              />
            )}
          </Fragment>
        ))}
        {insertingAt === lines.length && lineEditor(null)}
      </ol>
    </div>
  )
}
