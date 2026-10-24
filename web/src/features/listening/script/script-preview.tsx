import { useMutation } from '@tanstack/react-query'
import { useListeningPreviewScript } from '@/backend/listening/use-listening-preview-script'
import { ErrorText } from '@/components/error-text'
import { useDebounced } from '@/components/use-debounced'
import { Button } from '@/components/ui/button'
import { ScriptLines } from '@/features/listening/script/script-lines'
import type { ScriptTarget } from '@/features/listening/script/script-target'

type Props = {
  target: ScriptTarget
  text: string
  /** What saving means depends on where the script came from. */
  save: (text: string) => Promise<void>
  discard: () => void
}

/**
 * Every script is looked at here before it is stored: its title, summary and
 * numbered lines, or the problems that stop it. Only a passing script, the
 * one on screen, can be saved.
 */
export function ScriptPreview({ target, text, save, discard }: Props) {
  const settled = useDebounced(text, 250)
  const preview = useListeningPreviewScript(settled)
  const saving = useMutation({ mutationFn: save })
  const shown = preview.data
  const current = !preview.isPlaceholderData && settled === text
  const passed = shown?.kind === 'passed'

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-3">
      <div className="min-h-0 flex-1 overflow-y-auto">
        {text.trim() === '' ? (
          <p className="text-sm text-muted-foreground">无内容</p>
        ) : !shown ? (
          <p className="text-sm text-muted-foreground">加载中…</p>
        ) : shown.kind === 'failed' ? (
          <ul className="flex flex-col gap-1 text-sm text-destructive">
            {shown.problems.map((problem) => (
              <li key={problem}>{problem}</li>
            ))}
          </ul>
        ) : (
          <div className="flex flex-col gap-2">
            <p className="font-medium">
              {shown.title}
              {target.kind === 'append' && (
                <span className="ml-2 text-xs font-normal text-muted-foreground">续写时不用</span>
              )}
            </p>
            <p className="text-sm text-muted-foreground">{shown.summary}</p>
            <ScriptLines lines={shown.lines} />
          </div>
        )}
      </div>
      <div className="flex items-center justify-end gap-2">
        <ErrorText error={saving.error} />
        <Button variant="outline" disabled={saving.isPending} onClick={discard}>
          丢弃
        </Button>
        <Button
          disabled={!passed || !current || saving.isPending}
          onClick={() => saving.mutate(text)}
        >
          保存
        </Button>
      </div>
    </div>
  )
}
