import type { ReactNode } from 'react'
import { Link } from '@tanstack/react-router'
import { Page } from '@/components/page'
import { cn } from '@/components/ui/cn'

type Props = {
  tab: 'worlds' | 'characters'
  /** Buttons at the right end of the title row. */
  actions?: ReactNode
  children: ReactNode
}

const tabs = [
  { tab: 'worlds', to: '/listening/worlds', label: '世界' },
  { tab: 'characters', to: '/listening/characters', label: '角色' },
] as const

/** 听力 with its two tabs, worlds and characters, above the tab's content. */
export function ListeningPage({ tab, actions, children }: Props) {
  return (
    <Page title="听力" actions={actions}>
      <div className="-mt-2 flex gap-1 border-b">
        {tabs.map((item) => (
          <Link
            key={item.tab}
            to={item.to}
            className={cn(
              '-mb-px border-b-2 border-transparent px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground',
              item.tab === tab && 'border-foreground text-foreground',
            )}
          >
            {item.label}
          </Link>
        ))}
      </div>
      {children}
    </Page>
  )
}
