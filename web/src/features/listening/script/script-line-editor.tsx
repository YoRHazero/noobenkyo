import { useId } from 'react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { ProblemList } from '@/features/listening/script/problem-list'
import type { LineDraft } from '@/features/listening/script/script-edits'

type Props = {
  /** The line's number from 1; `null` for a line being added. */
  number: number | null
  draft: LineDraft
  problems: string[]
  /** The speakers already in the script, offered while typing one. */
  speakers: string[]
  busy: boolean
  /** False while nothing differs from the line as it is. */
  canConfirm: boolean
  onChange: (draft: LineDraft) => void
  onCancel: () => void
  onConfirm: () => void
}

/**
 * Editing one line of a script, or a line being added: who says it, the
 * Japanese with its reading marks, the Chinese. A confirm the check refuses
 * leaves the editor open with the problems under it.
 */
export function ScriptLineEditor({
  number,
  draft,
  problems,
  speakers,
  busy,
  canConfirm,
  onChange,
  onCancel,
  onConfirm,
}: Props) {
  const speakerList = useId()
  return (
    <li className="grid grid-cols-[2rem_1fr] gap-x-3 py-2">
      <span className="pt-2 text-right text-muted-foreground tabular-nums">{number ?? '新'}</span>
      <div className="flex flex-col gap-2">
        <Input
          aria-label="说话人"
          placeholder="说话人"
          list={speakerList}
          value={draft.speaker}
          disabled={busy}
          className="h-8 w-40"
          onChange={(event) => onChange({ ...draft, speaker: event.target.value })}
        />
        <datalist id={speakerList}>
          {speakers.map((speaker) => (
            <option key={speaker} value={speaker} />
          ))}
        </datalist>
        <Textarea
          lang="ja"
          aria-label="日语"
          placeholder="日语"
          spellCheck={false}
          value={draft.ja}
          disabled={busy}
          className="min-h-12"
          onChange={(event) => onChange({ ...draft, ja: event.target.value })}
        />
        <Textarea
          lang="zh"
          aria-label="中文"
          placeholder="中文"
          value={draft.zh}
          disabled={busy}
          className="min-h-12"
          onChange={(event) => onChange({ ...draft, zh: event.target.value })}
        />
        <ProblemList problems={problems} />
        <div className="flex justify-end gap-2">
          <Button variant="outline" size="sm" disabled={busy} onClick={onCancel}>
            取消
          </Button>
          <Button size="sm" disabled={busy || !canConfirm} onClick={onConfirm}>
            确定
          </Button>
        </div>
      </div>
    </li>
  )
}
