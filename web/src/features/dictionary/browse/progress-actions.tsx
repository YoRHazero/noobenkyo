import type { Progress, SenseKey } from '@/bindings'
import { useDictionaryUpdateProgress } from '@/backend/dictionary/use-dictionary-update-progress'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'
import { senseState } from '@/features/dictionary/sense-state'

type Props = {
  senseKey: SenseKey
  progress: Progress
  maxStage: number
}

/**
 * Reset to untouched, or mark as mastered the way skipping in study does:
 * stage N, the counts kept. Each acts on its second click, and is left out
 * where the sense already is.
 */
export function ProgressActions({ senseKey, progress, maxStage }: Props) {
  const update = useDictionaryUpdateProgress()
  const state = senseState(progress.stage, progress.passes, maxStage)
  return (
    <div className="flex items-center gap-1">
      {state !== 'untouched' && (
        <ConfirmButton
          label="重置"
          confirmLabel="确认重置"
          disabled={update.isPending}
          onConfirm={() => update.mutate({ key: senseKey, stage: 0, passes: 0, lapses: 0 })}
        />
      )}
      {state !== 'mastered' && (
        <ConfirmButton
          label="设为已掌握"
          confirmLabel="确认掌握"
          disabled={update.isPending}
          onConfirm={() =>
            update.mutate({
              key: senseKey,
              stage: maxStage,
              passes: progress.passes,
              lapses: progress.lapses,
            })
          }
        />
      )}
      <ErrorText error={update.error} />
    </div>
  )
}
