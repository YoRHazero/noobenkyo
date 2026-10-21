import type { Judgement, RevealPayload } from '@/bindings'
import { SentenceView } from '@/components/sentence-view'
import { cn } from '@/components/ui/cn'
import { Separator } from '@/components/ui/separator'
import { GlossList } from '@/features/study/quiz/gloss-list'

type Props = {
  reveal: RevealPayload
}

/** The answer: the sentence in full, the word, its glosses, and what was typed. */
export function RevealPanel({ reveal }: Props) {
  const { view, sentence } = reveal
  return (
    <div className="flex flex-col gap-3">
      <SentenceView fragments={sentence} className="text-2xl" />
      {view.translation && <p className="text-muted-foreground">{view.translation}</p>}
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
      {view.judgement && <AnswerLine input={view.input} judgement={view.judgement} />}
    </div>
  )
}

const tone: Record<Judgement, string> = {
  Correct: 'text-known',
  Wrong: 'text-unknown',
  Unsure: '',
}

function AnswerLine({ input, judgement }: { input: string; judgement: Judgement }) {
  return (
    <p className="text-sm">
      你的答案：
      <span className={cn(tone[judgement], !input && 'text-muted-foreground')}>
        {input || '（空）'}
      </span>
      {judgement === 'Unsure' && <span className="text-muted-foreground">（无法判断）</span>}
    </p>
  )
}
