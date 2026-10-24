import { useEffect, useState, useSyncExternalStore } from 'react'
import { LoaderCircle, Play, Square } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { clipState, playClip, stopClip, subscribeClip } from '@/features/listening/clip/clip-player'

type Props = {
  /** Tells this clip apart from every other one on the page. */
  clipKey: string
  load: () => Promise<Blob | null>
  label: string
  /** Sizes the button to the text around it. */
  className?: string
}

/** ▶ plays the clip, ■ stops it. Leaving the page stops it too. */
export function ClipButton({ clipKey, load, label, className }: Props) {
  const current = useSyncExternalStore(subscribeClip, clipState)
  const status = current?.key === clipKey ? current.status : null
  const [error, setError] = useState<string | null>(null)

  useEffect(
    () => () => {
      if (clipState()?.key === clipKey) {
        stopClip()
      }
    },
    [clipKey],
  )

  const toggle = () => {
    setError(null)
    if (status !== null) {
      stopClip()
      return
    }
    playClip(clipKey, load).catch((reason: unknown) =>
      setError(reason instanceof Error ? reason.message : String(reason)),
    )
  }

  return (
    <span className="inline-flex items-center gap-1">
      <Button
        variant="ghost"
        size="icon-sm"
        className={className}
        aria-label={status ? `停止${label}` : label}
        onClick={toggle}
      >
        {status === 'loading' ? (
          <LoaderCircle className="animate-spin" />
        ) : status === 'playing' ? (
          <Square />
        ) : (
          <Play />
        )}
      </Button>
      {error && <span className="text-xs text-destructive">{error}</span>}
    </span>
  )
}
