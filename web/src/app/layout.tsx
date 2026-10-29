import { Link, Outlet, useLocation } from '@tanstack/react-router'
import { cn } from '@/components/ui/cn'
import { AppNooChan } from '@/app/no-chan/app-no-chan'

/**
 * Learning and review are entered from the home page, so they are not here.
 * 听力 opens on the worlds and stays marked on every page under it. The
 * inbox is not here either: it is on no-chan's phone.
 */
const links = [
  { to: '/', label: '首页', section: null },
  { to: '/dictionary', label: '词库', section: null },
  { to: '/listening/worlds', label: '听力', section: '/listening/' },
  { to: '/models', label: '模型', section: null },
  { to: '/settings', label: '设置', section: null },
] as const

const itemClass =
  'rounded-md px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground'

/**
 * The top navigation, always shown, above the current page; no-chan at the
 * bottom. The theater, learning and review sessions are outside this
 * layout, so she is not there.
 */
export function Layout() {
  const pathname = useLocation({ select: (location) => location.pathname })
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
        </nav>
      </header>
      <main className="mx-auto w-full max-w-5xl flex-1 px-6 py-8">
        <Outlet />
      </main>
      <AppNooChan />
    </div>
  )
}
