import type { ReactNode } from 'react'

type Props = {
  title: string
  /** Said in the middle of the box while it has no episodes. */
  empty: string
  isEmpty: boolean
  /** The list, which scrolls inside the box. */
  children: ReactNode
}

/** One of the two boxes of the generation form, the same size on either side. */
export function EpisodeBox({ title, empty, isEmpty, children }: Props) {
  return (
    <div className="flex h-40 min-w-0 flex-col overflow-hidden rounded-md border">
      <p className="border-b px-3 py-1.5 text-xs text-muted-foreground">{title}</p>
      {isEmpty ? <p className="m-auto text-sm text-muted-foreground">{empty}</p> : children}
    </div>
  )
}
