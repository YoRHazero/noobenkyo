import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

/** Keeps showing the last preview while a new one is on its way. */
export function useListeningPreviewScript(text: string) {
  return useQuery({
    queryKey: ['listening_preview_script', text],
    queryFn: () => commands.listeningPreviewScript(text),
    placeholderData: keepPreviousData,
  })
}
