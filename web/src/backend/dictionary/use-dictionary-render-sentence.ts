import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { commands, type SentenceDisplay } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Keeps showing the last rendering while the text is being typed. */
export function useDictionaryRenderSentence(marked: string, display: SentenceDisplay) {
  return useQuery({
    queryKey: ['dictionary_render_sentence', marked, display],
    queryFn: () => unwrap(commands.dictionaryRenderSentence(marked, display)),
    placeholderData: keepPreviousData,
  })
}
