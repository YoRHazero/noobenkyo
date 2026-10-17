import type { QuestionPayload, Zone } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { AnswerForm } from '@/features/study/answer-form'
import { ComprehensiveCard } from '@/features/study/comprehensive-card'
import { GlossList } from '@/features/study/gloss-list'
import { questionTypeLabel } from '@/features/study/labels'
import { QuestionActions } from '@/features/study/question-actions'
import { RatingBar } from '@/features/study/rating-bar'
import { RevealPanel } from '@/features/study/reveal-panel'
import { SentenceView } from '@/features/study/sentence-view'
import { StepDots } from '@/features/study/step-dots'
import { useSubmitAnswer } from '@/features/study/use-submit-answer'

type Props = {
  zone: Zone
  question: QuestionPayload
  onEnded: () => void
}

/** One question: the prompt and answer box, then the revealed answer and rating. */
export function QuestionCard({ zone, question, onEnded }: Props) {
  const { view, fragments } = question
  const submit = useSubmitAnswer(zone)
  const reveal = submit.data

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle>{questionTypeLabel[view.question_type]}</CardTitle>
        <StepDots remaining={view.remaining} steps={view.steps} />
      </CardHeader>
      <CardContent className="flex flex-col gap-5">
        {reveal === undefined ? (
          <>
            {fragments && <SentenceView fragments={fragments} />}
            {view.glosses && <GlossList glosses={view.glosses} />}
            {view.card && <ComprehensiveCard card={view.card} />}
            <AnswerForm
              input={view.answer_input}
              pending={submit.isPending}
              onSubmit={submit.mutate}
            />
            <ErrorText error={submit.error} />
          </>
        ) : (
          <>
            <RevealPanel reveal={reveal} />
            <RatingBar zone={zone} prediction={reveal.view.prediction} onEnded={onEnded} />
          </>
        )}
        <QuestionActions
          zone={zone}
          showsSentence={fragments !== null || (reveal?.fragments ?? null) !== null}
          onEnded={onEnded}
        />
      </CardContent>
    </Card>
  )
}
