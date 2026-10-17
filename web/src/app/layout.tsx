import { Link, Outlet } from '@tanstack/react-router'

const links = [
  { to: '/', label: '首页' },
  { to: '/learn', label: '学习' },
  { to: '/review', label: '复习' },
  { to: '/dictionary', label: '词库' },
  { to: '/settings', label: '设置' },
] as const

export function Layout() {
  return (
    <div className="flex min-h-screen flex-col">
      <header className="border-b bg-card">
        <nav className="mx-auto flex max-w-4xl items-center gap-1 px-6 py-3">
          <span className="mr-6 font-semibold tracking-wide">noobenkyo</span>
          {links.map((link) => (
            <Link
              key={link.to}
              to={link.to}
              className="rounded-md px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
              activeProps={{ className: 'bg-accent text-foreground' }}
              activeOptions={{ exact: true }}
            >
              {link.label}
            </Link>
          ))}
        </nav>
      </header>
      <main className="mx-auto w-full max-w-4xl flex-1 px-6 py-8">
        <Outlet />
      </main>
    </div>
  )
}
