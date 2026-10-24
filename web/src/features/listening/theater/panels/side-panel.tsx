import type { ReactNode } from 'react'
import { X } from 'lucide-react'
import { StageButton } from '@/features/listening/theater/stage/stage-button'

type Props = {
  title: string
  onClose: () => void
  children: ReactNode
}

/** A panel over the right of the stage; playing goes on underneath. */
export function SidePanel({ title, onClose, children }: Props) {
  return (
    <aside className="absolute top-0 right-0 bottom-0 z-20 flex w-[min(34rem,90vw)] flex-col border-l border-white/10 bg-black/75 text-white backdrop-blur-xl motion-safe:animate-in motion-safe:slide-in-from-right-8 motion-safe:fade-in motion-safe:duration-200">
      <div className="flex items-center justify-between px-5 py-3">
        <h2 className="text-sm text-white/70">{title}</h2>
        <StageButton label="关闭（Esc）" onClick={onClose}>
          <X />
        </StageButton>
      </div>
      <div className="min-h-0 flex-1 overflow-y-auto px-5 pb-5">{children}</div>
    </aside>
  )
}
