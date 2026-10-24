import type { ReactNode } from 'react'

type Props = {
  shown: boolean
  label: string
  onReveal: () => void
  children: ReactNode
}

/** One language of the line, or a bar standing in for it until it is revealed. */
export function SubtitleRow({ shown, label, onReveal, children }: Props) {
  if (shown) {
    return <>{children}</>
  }
  return (
    <button
      type="button"
      aria-label={`显示${label}`}
      className="my-2 h-5 w-2/3 rounded bg-white/10 transition-colors hover:bg-white/20"
      onMouseDown={(event) => event.preventDefault()}
      onClick={onReveal}
    />
  )
}
