import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'

type Props = {
  changed: boolean
  /** Both times are whole seconds, so there is something to send. */
  complete: boolean
  busy: boolean
  error: Error | null
  onRevert: () => void
  onSave: () => void
}

/** Stays at the bottom: what is unsaved, and the buttons that act on it. */
export function ModelsSaveBar({ changed, complete, busy, error, onRevert, onSave }: Props) {
  return (
    <div className="sticky bottom-0 flex items-center gap-2 border-t bg-background py-3">
      <div className="mr-auto text-sm">
        {error ? (
          <ErrorText error={error} />
        ) : (
          changed && <span className="text-muted-foreground">有未保存的改动</span>
        )}
      </div>
      <Button variant="outline" size="sm" disabled={!changed || busy} onClick={onRevert}>
        还原
      </Button>
      <Button size="sm" disabled={!changed || !complete || busy} onClick={onSave}>
        保存
      </Button>
    </div>
  )
}
