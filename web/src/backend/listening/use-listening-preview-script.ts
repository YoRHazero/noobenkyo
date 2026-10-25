import { keepPreviousData, useQuery, useQueryClient } from '@tanstack/react-query'
import { commands } from '@/bindings'

const previewQuery = (text: string) => ({
  queryKey: ['listening_preview_script', text],
  queryFn: () => commands.listeningPreviewScript(text),
})

/** Keeps showing the last preview while a new one is on its way. */
export function useListeningPreviewScript(text: string) {
  return useQuery({ ...previewQuery(text), placeholderData: keepPreviousData })
}

/**
 * Previews a text on demand, through the same cache: a text checked before
 * it is shown is shown at once.
 */
export function useFetchListeningPreviewScript() {
  const queryClient = useQueryClient()
  return (text: string) => queryClient.fetchQuery(previewQuery(text))
}
