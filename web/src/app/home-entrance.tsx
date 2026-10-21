import { Link } from '@tanstack/react-router'

type Props = {
  to: '/learn' | '/review'
  title: string
  label: string
  /** `undefined` while the counts are loading. */
  count: number | undefined
}

/** One entrance on the home page: a card leading to its page. */
export function HomeEntrance({ to, title, label, count }: Props) {
  return (
    <Link
      to={to}
      className="flex flex-col gap-3 rounded-xl border bg-card p-6 shadow-sm transition-shadow hover:shadow-md"
    >
      <span className="text-xl font-semibold">{title}</span>
      <span className="text-sm text-muted-foreground">
        {label}{' '}
        <span className="text-2xl font-semibold text-foreground tabular-nums">{count ?? '—'}</span>
      </span>
    </Link>
  )
}
