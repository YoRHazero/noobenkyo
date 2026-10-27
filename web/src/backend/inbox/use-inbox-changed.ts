import { useEffect } from 'react'
import { useQueryClient } from '@tanstack/react-query'
import { events } from '@/bindings'

/**
 * Reads the inbox again whenever it changes. The event does not say what
 * changed or who changed it, so every inbox query is refetched.
 */
export function useInboxChanged() {
  const queryClient = useQueryClient()
  useEffect(() => {
    const stopping = events.inboxChanged.listen(() =>
      queryClient.invalidateQueries({
        predicate: (query) => String(query.queryKey[0]).startsWith('inbox_'),
      }),
    )
    return () => {
      void stopping.then((stop) => stop())
    }
  }, [queryClient])
}
