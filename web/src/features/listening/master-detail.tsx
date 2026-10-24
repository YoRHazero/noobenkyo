import type { ReactNode } from 'react'

type Props = {
  list: ReactNode
  detail: ReactNode
}

/** A list of names on the left and the chosen one on the right. */
export function MasterDetail({ list, detail }: Props) {
  return (
    <div className="grid grid-cols-[11rem_1fr] gap-6">
      <nav className="flex flex-col gap-0.5">{list}</nav>
      <div className="min-w-0">{detail}</div>
    </div>
  )
}
