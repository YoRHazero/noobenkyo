import type { Rating, RevealPayload } from '@/bindings'
import { Separator } from '@/components/ui/separator'
import { GlossList } from '@/features/study/gloss-list'
import { SentenceView } from '@/features/study/sentence-view'

type Props = {
  reveal: RevealPayload
}

/** The answer after submitting: the word, its glosses, the sentence and the comparison. */
export function RevealPanel({ reveal }: Props) {
  const { view, fragments } = reveal
  return (
    <div className="flex flex-col gap-3">
      <Separator />
      <div className="flex items-baseline gap-4">
        <span lang="ja" className="text-3xl">
          {view.writing}
        </span>
        <span lang="ja" className="text-lg text-muted-foreground">
          {view.kana}
        </span>
      </div>
      <GlossList glosses={view.glosses} />
      {fragments && <SentenceView fragments={fragments} />}
      {view.translation && <p className="text-muted-foreground">{view.translation}</p>}
      <AnswerComparison input={view.input} expected={view.expected} prediction={view.prediction} />
    </div>
  )
}

type ComparisonProps = {
  input: string
  expected: string | null
  prediction: Rating | null
}

/** The input against the expected answer; whether it matched is the backend's prediction. */
function AnswerComparison({ input, expected, prediction }: ComparisonProps) {
  if (expected === null) {
    return input ? <p className="text-sm">你的答案：{input}</p> : null
  }
  const matched = prediction === 'Known'
  return (
    <p className="text-sm">
      你的答案：
      <span lang="ja" className={matched ? 'text-known' : 'text-unknown'}>
        {input || '（空）'}
      </span>
      {!matched && (
        <>
          ，应为：<span lang="ja">{expected}</span>
        </>
      )}
    </p>
  )
}
