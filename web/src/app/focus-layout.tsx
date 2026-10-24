import { Outlet } from '@tanstack/react-router'

/**
 * Learning and a review session take the whole window: no navigation, only
 * the quiz and the page's own way out.
 */
export function FocusLayout() {
  return (
    <main className="mx-auto min-h-screen w-full max-w-5xl px-6 py-8">
      <Outlet />
    </main>
  )
}
