import type { Fragment } from '@/bindings'

type Props = {
  fragments: Fragment[]
}

/** A rendered sentence: ruby on kanji, the target highlighted or left blank. */
export function SentenceView({ fragments }: Props) {
  return (
    <p lang="ja" className="text-2xl leading-[2.4]">
      {fragments.map((fragment, index) => (
        <FragmentView key={index} fragment={fragment} />
      ))}
    </p>
  )
}

function FragmentView({ fragment }: { fragment: Fragment }) {
  switch (fragment.kind) {
    case 'text':
      return <span>{fragment.text}</span>
    case 'ruby':
      return (
        <ruby>
          {fragment.base}
          <rt>{fragment.ruby}</rt>
        </ruby>
      )
    case 'target':
      return (
        <span className="font-semibold text-target underline decoration-2 underline-offset-8">
          {fragment.text}
        </span>
      )
    case 'blank':
      return (
        <span className="mx-1 inline-block w-24 border-b-2 border-target align-baseline">
          &nbsp;
        </span>
      )
  }
}
