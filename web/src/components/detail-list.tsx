import type { ReactNode } from 'react'

/** A row's detail: labels on the left, what they name on the right. */
export function DetailList({ children }: { children: ReactNode }) {
  return (
    <dl className="grid grid-cols-[minmax(4rem,auto)_1fr] gap-x-4 gap-y-2 text-sm">{children}</dl>
  )
}

/** One labelled item of a `DetailList`. */
export function DetailItem({ label, children }: { label: string; children: ReactNode }) {
  return (
    <>
      <dt className="text-muted-foreground">{label}</dt>
      <dd>{children}</dd>
    </>
  )
}
