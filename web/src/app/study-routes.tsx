import { getRouteApi } from '@tanstack/react-router'
import { ReviewSessionPage } from '@/features/study/review/review-session-page'

/** The review session, given its mode from the address. */

const session = getRouteApi('/focus/review/session')

export function ReviewSessionRoute() {
  return <ReviewSessionPage mode={session.useSearch().mode} />
}
