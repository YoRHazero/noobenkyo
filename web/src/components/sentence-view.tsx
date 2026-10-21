import type { Fragment, TargetPiece } from '@/bindings'
import { cn } from '@/components/ui/cn'

type Props = {
  fragments: Fragment[]
  /** Sets the size; ruby scales with the text. */
  className?: string
}

/**
 * A rendered sentence: ruby where the sentence has it, the target marked as
 * one unit, or left blank. What is shown is decided by the backend.
 */
export function SentenceView({ fragments, className }: Props) {
  return (
    <span lang="ja" className={cn('leading-[2.2]', className)}>
      {fragments.map((fragment, index) => (
        <FragmentView key={index} fragment={fragment} />
      ))}
    </span>
  )
}

function FragmentView({ fragment }: { fragment: Fragment }) {
  switch (fragment.kind) {
    case 'text':
      return <>{fragment.text}</>
    case 'ruby':
      return <Ruby base={fragment.base} ruby={fragment.ruby} />
    case 'target':
      return (
        <span className="font-semibold text-target">
          {fragment.pieces.map((piece, index) => (
            <PieceView key={index} piece={piece} />
          ))}
        </span>
      )
    case 'blank':
      return <span className="mx-1 inline-block w-[4em] border-b-2 border-target align-baseline" />
  }
}

function PieceView({ piece }: { piece: TargetPiece }) {
  return piece.kind === 'text' ? <>{piece.text}</> : <Ruby base={piece.base} ruby={piece.ruby} />
}

function Ruby({ base, ruby }: { base: string; ruby: string }) {
  return (
    <ruby>
      {base}
      <rt>{ruby}</rt>
    </ruby>
  )
}
