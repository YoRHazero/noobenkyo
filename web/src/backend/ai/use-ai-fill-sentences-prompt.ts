import { useQuery } from '@tanstack/react-query'
import { commands, type FillSentencesTask } from '@/bindings'

/**
 * The prompt for filling in sentences for the rows the filter shows. It is
 * worked out as the form changes, so a click can put it on the clipboard
 * at once.
 */
export function useAiFillSentencesPrompt(task: FillSentencesTask) {
  return useQuery({
    queryKey: ['ai_fill_sentences_prompt', task],
    queryFn: () => commands.aiFillSentencesPrompt(task),
  })
}
