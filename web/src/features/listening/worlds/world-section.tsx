import type { ReactNode } from 'react'

type Props = {
  title: string
  /** Buttons at the right end of the section's title. */
  actions?: ReactNode
  children: ReactNode
}

/** One block of a world: its episodes, or its cast. */
export function WorldSection({ title, actions, children }: Props) {
  return (
    <section className="flex flex-col gap-2">
      <div className="flex items-center justify-between gap-2">
        <h3 className="text-sm font-medium text-muted-foreground">{title}</h3>
        {actions && <div className="flex gap-2">{actions}</div>}
      </div>
      {children}
    </section>
  )
}
