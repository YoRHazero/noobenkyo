import { useRef } from 'react'
import { Reorder } from 'motion/react'
import type { EpisodeSummary } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { episodeLabel } from '@/features/listening/episode-label'

type Props = {
  episode: EpisodeSummary
  picked: boolean
  /** In the chosen box, where it is dragged into order; it must sit in a Reorder.Group. */
  reorderable: boolean
  onToggle: () => void
  /** Double-clicked: moves to the other box at once. */
  onMove: () => void
}

/**
 * An episode in either box: a click (or Space, Enter) picks it for the
 * arrows, a double click moves it across. A drag in the chosen box only
 * reorders, without picking.
 */
export function EpisodeOption({ episode, picked, reorderable, onToggle, onMove }: Props) {
  // The click that may follow a drag is not a pick.
  const dragged = useRef(false)
  const shared = {
    role: 'option',
    'aria-selected': picked,
    tabIndex: 0,
    className: cn(
      'flex items-center rounded-sm px-2 py-1 text-sm select-none hover:bg-accent/60',
      // Opaque, so a dragged one covers those it passes.
      reorderable && 'cursor-grab bg-background active:cursor-grabbing',
      picked && 'bg-accent hover:bg-accent',
    ),
    onPointerDown: () => {
      dragged.current = false
    },
    onClick: () => {
      if (!dragged.current) {
        onToggle()
      }
    },
    onDoubleClick: onMove,
    onKeyDown: (event: React.KeyboardEvent) => {
      if (event.key === ' ' || event.key === 'Enter') {
        event.preventDefault()
        onToggle()
      }
    },
  }
  const name = (
    <span className="min-w-0 truncate">
      <span className="mr-2 text-muted-foreground tabular-nums">
        {episodeLabel(episode.number)}
      </span>
      {episode.title}
    </span>
  )
  return reorderable ? (
    <Reorder.Item
      value={episode.uuid}
      whileDrag={{ scale: 1.02, boxShadow: '0 6px 16px rgb(0 0 0 / 0.15)' }}
      onDragStart={() => {
        dragged.current = true
      }}
      {...shared}
    >
      {name}
    </Reorder.Item>
  ) : (
    <li {...shared}>{name}</li>
  )
}
