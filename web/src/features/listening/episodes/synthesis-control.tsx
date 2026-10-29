import { useSpeakLinesTask } from '@/backend/ai/ai-tasks'
import { useAiCancel } from '@/backend/ai/use-ai-cancel'
import { useAiStartSpeakLines } from '@/backend/ai/use-ai-start-speak-lines'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'

type Props = {
  episode: string
}

/**
 * Synthesizes what the episode lacks. While it runs: how far it has got,
 * and the way to stop it; what is stored by then stays. After a failure,
 * why it failed, until the next run.
 */
export function SynthesisControl({ episode }: Props) {
  const task = useSpeakLinesTask(episode)
  const start = useAiStartSpeakLines()
  const cancel = useAiCancel()

  if (task && task.outcome === null) {
    return (
      <div className="flex items-center gap-2">
        <span className="text-sm text-muted-foreground tabular-nums">
          合成：{task.total === null ? '…' : `${task.done}/${task.total}`}
        </span>
        <Button
          variant="outline"
          size="sm"
          disabled={cancel.isPending}
          onClick={() => cancel.mutate(task.id)}
        >
          取消
        </Button>
        <ErrorText error={cancel.error} />
      </div>
    )
  }
  return (
    <div className="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={start.isPending}
        onClick={() => start.mutate(episode)}
      >
        合成
      </Button>
      <ErrorText error={start.error} />
      {task?.outcome?.kind === 'failed' && (
        <p className="text-sm text-destructive">{task.outcome.message}</p>
      )}
    </div>
  )
}
