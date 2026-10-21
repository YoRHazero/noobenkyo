import type { QuizPayload } from '@/bindings'
import { SentenceView } from '@/components/sentence-view'
import { ComprehensiveCard } from '@/features/study/quiz/comprehensive-card'
import { GlossList } from '@/features/study/quiz/gloss-list'

type Props = {
  quiz: QuizPayload
}

/**
 * What the question shows before answering. Which parts there are, and how
 * the sentence shows its target, is decided by the backend.
 */
export function QuizFace({ quiz }: Props) {
  const { view, sentence } = quiz
  return (
    <div className="flex flex-col gap-4">
      {sentence && <SentenceView fragments={sentence} className="text-2xl" />}
      {view.glosses && <GlossList glosses={view.glosses} />}
      {view.card && <ComprehensiveCard card={view.card} />}
    </div>
  )
}
