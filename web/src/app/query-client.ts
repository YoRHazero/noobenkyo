import { QueryClient } from '@tanstack/react-query'

/** Data lives in the backend; queries refetch when pages mount and never retry silently. */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: { retry: false, refetchOnWindowFocus: false },
    mutations: { retry: false },
  },
})
