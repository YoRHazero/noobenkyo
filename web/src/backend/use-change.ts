import { useMutation, useQueryClient } from '@tanstack/react-query'

/**
 * A command that changes data. On success every cached result is
 * invalidated: which reads a change affects is the backend's business, so
 * the frontend refetches what is on screen instead of guessing. The change
 * stays pending until those refetches are done, so the page never shows the
 * old result after a change.
 */
export function useChange<Input, Output>(run: (input: Input) => Promise<Output>) {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: run,
    onSuccess: () => queryClient.invalidateQueries(),
  })
}
