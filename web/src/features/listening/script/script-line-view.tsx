import type { ReactNode } from 'react'
import type { RubySegment } from '@/bindings'
import { RubyLine } from '@/features/listening/ruby-line'

type Props = {
  /** From 1, as the checks number lines. */
  number: number
  line: {
    speaker: string
    ja_ruby: RubySegment[]
    zh: string
  }
  /** Buttons at the end of the line. */
  actions: ReactNode
}

/** One line of a script as it will be stored: who says it, the Japanese with ruby, the Chinese. */
export function ScriptLineView({ number, line, actions }: Props) {
  return (
    <li className="grid grid-cols-[2rem_5rem_1fr_auto] items-start gap-x-3 py-2">
      <span className="pt-1 text-right text-muted-foreground tabular-nums">{number}</span>
      <span className="truncate pt-1 font-medium">{line.speaker}</span>
      <div className="flex min-w-0 flex-col">
        <RubyLine segments={line.ja_ruby} className="text-base" />
        <span className="text-muted-foreground">{line.zh}</span>
      </div>
      <div className="flex items-center">{actions}</div>
    </li>
  )
}
