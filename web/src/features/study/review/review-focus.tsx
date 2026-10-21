import type { ReactNode } from 'react'
import { Dialog, DialogContent, DialogTitle } from '@/components/ui/dialog'

/**
 * A session takes the whole window: no navigation, and nothing behind it can
 * be reached. Escape does nothing; the session's own buttons are the way out.
 * Opening focuses the window itself, not its first button: Enter rates an
 * answer, and must not press 结束.
 */
export function ReviewFocus({ children }: { children: ReactNode }) {
  const stay = (event: Event) => event.preventDefault()
  return (
    <Dialog open>
      <DialogContent
        aria-describedby={undefined}
        showCloseButton={false}
        onOpenAutoFocus={(event) => {
          event.preventDefault()
          if (event.currentTarget instanceof HTMLElement) {
            event.currentTarget.focus()
          }
        }}
        onEscapeKeyDown={stay}
        onInteractOutside={stay}
        className="top-0 left-0 block h-screen w-screen max-w-none translate-x-0 translate-y-0 overflow-y-auto rounded-none border-0 p-0 shadow-none sm:max-w-none"
      >
        <DialogTitle className="sr-only">复习</DialogTitle>
        <div className="mx-auto w-full max-w-5xl px-6 py-8">{children}</div>
      </DialogContent>
    </Dialog>
  )
}
