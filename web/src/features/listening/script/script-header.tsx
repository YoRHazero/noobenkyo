import { Pencil } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { ProblemList } from '@/features/listening/script/problem-list'
import type { HeaderDraft } from '@/features/listening/script/script-edits'

type Props = {
  title: string
  summary: string
  /** Continuing an episode keeps its title, so the script's title is not used or edited. */
  titleUsed: boolean
  /** What the open header editor holds; `null` while it is closed. */
  editing: { draft: HeaderDraft; problems: string[] } | null
  canEdit: boolean
  busy: boolean
  canConfirm: boolean
  onEdit: () => void
  onChange: (draft: HeaderDraft) => void
  onCancel: () => void
  onConfirm: () => void
}

/** The script's title and summary, edited together. */
export function ScriptHeader({
  title,
  summary,
  titleUsed,
  editing,
  canEdit,
  busy,
  canConfirm,
  onEdit,
  onChange,
  onCancel,
  onConfirm,
}: Props) {
  if (!editing) {
    return (
      <div className="flex items-start gap-2">
        <div className="flex min-w-0 flex-1 flex-col gap-1">
          <p className="font-medium">
            {title}
            {!titleUsed && (
              <span className="ml-2 text-xs font-normal text-muted-foreground">续写时不用</span>
            )}
          </p>
          <p className="text-sm text-muted-foreground">{summary}</p>
        </div>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="编辑"
          disabled={!canEdit}
          onClick={onEdit}
        >
          <Pencil />
        </Button>
      </div>
    )
  }
  const { draft, problems } = editing
  return (
    <div className="flex flex-col gap-2">
      {titleUsed ? (
        <Input
          aria-label="标题"
          placeholder="标题"
          value={draft.title}
          disabled={busy}
          className="h-8"
          onChange={(event) => onChange({ ...draft, title: event.target.value })}
        />
      ) : (
        <p className="font-medium">
          {title}
          <span className="ml-2 text-xs font-normal text-muted-foreground">续写时不用</span>
        </p>
      )}
      <Textarea
        aria-label="梗概"
        placeholder="梗概"
        value={draft.summary}
        disabled={busy}
        onChange={(event) => onChange({ ...draft, summary: event.target.value })}
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
  )
}
