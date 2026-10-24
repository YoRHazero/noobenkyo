import type { RubySegment } from '@/bindings'
import { cn } from '@/components/ui/cn'

type Props = {
  segments: RubySegment[]
  /** Sets the size; ruby scales with the text. */
  className?: string
}

/** A Japanese line as listening split it: plain text, and ruby over its marked words. */
export function RubyLine({ segments, className }: Props) {
  return (
    <span lang="ja" className={cn('leading-[2.2]', className)}>
      {segments.map((segment, index) =>
        segment.kind === 'text' ? (
          <span key={index}>{segment.text}</span>
        ) : (
          <ruby key={index}>
            {segment.writing}
            <rt>{segment.reading}</rt>
          </ruby>
        ),
      )}
    </span>
  )
}
