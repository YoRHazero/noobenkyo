import { useState, type ReactNode } from 'react'

type Props = {
  shown: boolean
  children: ReactNode
}

/** Content covered until clicked, unless shown from the start. */
export function Covered({ shown, children }: Props) {
  const [opened, setOpened] = useState(false)
  if (shown || opened) {
    return <>{children}</>
  }
  return (
    <button
      type="button"
      className="rounded-md bg-muted px-4 py-1 text-sm text-muted-foreground hover:bg-accent"
      onClick={() => setOpened(true)}
    >
      显示
    </button>
  )
}
