import { useState } from 'react'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ClearZoneDialog } from '@/features/settings/clear-zone-dialog'
import type { ZoneSetting } from '@/features/settings/settings-draft'

type Props = {
  changed: boolean
  /** Every number is whole, so there is something to send. */
  complete: boolean
  atDefaults: boolean
  /** What clears the learning zone; saving asks first when there is any. */
  zoneChanges: ZoneSetting[]
  busy: boolean
  error: Error | null
  onDefaults: () => void
  onRevert: () => void
  onSave: () => void
}

/** Stays at the bottom: what is unsaved, and the buttons that act on it. */
export function SaveBar({ changed, complete, atDefaults, zoneChanges, busy, error, ...on }: Props) {
  const [asking, setAsking] = useState(false)
  const savable = changed && complete && !busy
  return (
    <div className="sticky bottom-0 flex items-center gap-2 border-t bg-background py-3">
      <div className="mr-auto text-sm">
        {error ? (
          <ErrorText error={error} />
        ) : (
          changed && <span className="text-muted-foreground">有未保存的改动</span>
        )}
      </div>
      <Button variant="outline" size="sm" disabled={atDefaults || busy} onClick={on.onDefaults}>
        恢复默认
      </Button>
      <Button variant="outline" size="sm" disabled={!changed || busy} onClick={on.onRevert}>
        还原
      </Button>
      <Button
        size="sm"
        disabled={!savable}
        onClick={() => (zoneChanges.length > 0 ? setAsking(true) : on.onSave())}
      >
        保存
      </Button>
      <ClearZoneDialog
        changes={zoneChanges}
        open={asking}
        onOpenChange={setAsking}
        onSave={on.onSave}
      />
    </div>
  )
}
