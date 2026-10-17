import { useState } from 'react'
import type { Zone } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { EmptyZone } from '@/features/study/empty-zone'
import { zoneLabel } from '@/features/study/labels'
import { QuestionCard } from '@/features/study/question-card'
import { ReviewModeSwitch } from '@/features/study/review-mode-switch'
import { useCurrentQuestion } from '@/features/study/use-current-question'
import { useZoneStatus } from '@/features/study/use-zone-status'
import { ZoneSummary } from '@/features/study/zone-summary'

type Props = {
  zone: Zone
}

/** The learning or review zone: one question at a time. */
export function ZonePage({ zone }: Props) {
  const [round, setRound] = useState(0)
  const next = () => setRound((value) => value + 1)
  const status = useZoneStatus(zone)
  const question = useCurrentQuestion(zone, round)

  const modeSwitch =
    zone === 'Review' && status.data ? (
      <ReviewModeSwitch mode={status.data.review_mode} onChanged={next} />
    ) : null

  return (
    <Page title={zoneLabel[zone]} actions={modeSwitch}>
      {status.data && <ZoneSummary status={status.data} />}
      <ErrorText error={status.error ?? question.error} />
      {question.data && (
        <QuestionCard key={round} zone={zone} question={question.data} onEnded={next} />
      )}
      {question.data === null && status.data && <EmptyZone status={status.data} onStarted={next} />}
    </Page>
  )
}
