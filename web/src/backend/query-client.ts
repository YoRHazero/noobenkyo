import { QueryClient } from '@tanstack/react-query'

/**
 * The one cache of command results. The data lives in the backend: a failed
 * command is shown as it failed, never retried, and refocusing the window
 * does not refetch. Changes invalidate everything (see `useChange`).
 */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: { retry: false, refetchOnWindowFocus: false },
    mutations: { retry: false },
  },
})
