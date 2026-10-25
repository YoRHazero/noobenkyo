import { lazy, Suspense, useState } from 'react'
import { Link, Outlet, useLocation } from '@tanstack/react-router'
import { cn } from '@/components/ui/cn'
import { InboxMenu } from '@/app/inbox/inbox-menu'

/**
 * Loaded apart from the pages, frames and all. The theater, learning and
 * review sessions are outside this layout, so they never load her.
 */
const Pet = lazy(() => import('@/features/pet/pet').then((module) => ({ default: module.Pet })))

/**
 * Learning and review are entered from the home page, so they are not here.
 * 听力 opens on the worlds and stays marked on every page under it. The
 * inbox sits at the far end.
 */
const links = [
  { to: '/', label: '首页', section: null },
  { to: '/dictionary', label: '词库', section: null },
  { to: '/listening/worlds', label: '听力', section: '/listening/' },
  { to: '/settings', label: '设置', section: null },
] as const

const itemClass =
  'rounded-md px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground'

/** The top navigation, always shown, above the current page; the pet at the bottom. */
export function Layout() {
  const pathname = useLocation({ select: (location) => location.pathname })
  const [inboxOpen, setInboxOpen] = useState(false)
  return (
    <div className="flex min-h-screen flex-col">
      <header className="border-b bg-card">
        <nav className="mx-auto flex max-w-5xl items-center gap-1 px-6 py-3">
          <span className="mr-6 font-semibold tracking-wide">noobenkyo</span>
          {links.map((link) => {
            const active =
              link.section === null ? pathname === link.to : pathname.startsWith(link.section)
            return (
              <Link
                key={link.to}
                to={link.to}
                className={cn(itemClass, active && 'bg-accent text-foreground')}
              >
                {link.label}
              </Link>
            )
          })}
          <div className="ml-auto">
            <InboxMenu className={itemClass} open={inboxOpen} onOpenChange={setInboxOpen} />
          </div>
        </nav>
      </header>
      <main className="mx-auto w-full max-w-5xl flex-1 px-6 py-8">
        <Outlet />
      </main>
      <Suspense fallback={null}>
        <Pet onOpenInbox={() => setInboxOpen(true)} />
      </Suspense>
    </div>
  )
}
