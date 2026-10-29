import { useEffect } from 'react'
import { useQueryClient } from '@tanstack/react-query'
import { events } from '@/bindings'
import { aiTaskFinished, aiTaskProgressed } from '@/backend/ai/ai-tasks'

/**
 * Follows every task's events into the task store. Each item a task stores,
 * and its end, may change what any page shows, so every cached result is
 * refetched then, as after a change. Mounted once, at the root, so no event
 * is missed whatever page is open.
 */
export function useAiTaskEvents(onFailed: () => void) {
  const queryClient = useQueryClient()
  useEffect(() => {
    const stopping = [
      events.aiTaskProgress.listen(({ payload }) => {
        aiTaskProgressed(payload)
        if (payload.done > 0) {
          void queryClient.invalidateQueries()
        }
      }),
      events.aiTaskFinished.listen(({ payload }) => {
        aiTaskFinished(payload)
        void queryClient.invalidateQueries()
        if (payload.outcome.kind === 'failed') {
          onFailed()
        }
      }),
    ]
    return () => {
      for (const stop of stopping) {
        void stop.then((unlisten) => unlisten())
      }
    }
  }, [queryClient, onFailed])
}
