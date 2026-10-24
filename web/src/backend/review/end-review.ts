import { commands } from '@/bindings'
import { queryClient } from '@/backend/query-client'
import { unwrap } from '@/backend/unwrap'

/**
 * Ends the review session. Called by the router when the session page is
 * left, outside React, so it invalidates the cache itself as a change would.
 */
export async function endReview(): Promise<void> {
  await unwrap(commands.reviewEnd())
  await queryClient.invalidateQueries()
}
