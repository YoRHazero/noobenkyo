import type { ReactNode } from 'react'

type Props = {
  title: string
  actions?: ReactNode
  children: ReactNode
}

/** A page with a title row and stacked sections. */
export function Page({ title, actions, children }: Props) {
  return (
    <div className="flex flex-col gap-6">
      <div className="flex items-center justify-between gap-4">
        <h1 className="text-2xl font-semibold">{title}</h1>
        {actions}
      </div>
      {children}
    </div>
  )
}
