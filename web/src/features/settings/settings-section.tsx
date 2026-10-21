import type { ReactNode } from 'react'

/** A group of settings under its heading. */
export function SettingsSection({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="flex flex-col">
      <h2 className="mb-1 font-semibold">{title}</h2>
      <div className="flex flex-col divide-y">{children}</div>
    </section>
  )
}
