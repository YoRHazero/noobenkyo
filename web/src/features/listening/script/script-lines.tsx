import type { RubySegment } from '@/bindings'
import { RubyLine } from '@/features/listening/ruby-line'

type Line = {
  speaker: string
  ja_ruby: RubySegment[]
  zh: string
}

type Props = {
  lines: Line[]
}

/**
 * A script's lines, numbered from 1 as the checks number them. Read only:
 * editing in the preview comes with the inbox.
 */
export function ScriptLines({ lines }: Props) {
  return (
    <ol className="flex flex-col divide-y text-sm">
      {lines.map((line, index) => (
        <li key={index} className="grid grid-cols-[2rem_5rem_1fr] gap-x-3 py-2">
          <span className="text-right text-muted-foreground tabular-nums">{index + 1}</span>
          <span className="truncate font-medium">{line.speaker}</span>
          <div className="flex flex-col">
            <RubyLine segments={line.ja_ruby} className="text-base" />
            <span className="text-muted-foreground">{line.zh}</span>
          </div>
        </li>
      ))}
    </ol>
  )
}
