import { useState, type ReactNode } from 'react'

type Props = {
  shown: boolean
  children: ReactNode
}

/** Content that stays covered until clicked, unless `shown`. */
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
      点击显示
    </button>
  )
}
