import { useQuery } from '@tanstack/react-query'
import { commands, type WordListTask } from '@/bindings'

/**
 * The prompt for writing a new word list. It is worked out as the form
 * changes, so a click can put it on the clipboard at once.
 */
export function useAiWordListPrompt(task: WordListTask) {
  return useQuery({
    queryKey: ['ai_word_list_prompt', task],
    queryFn: () => commands.aiWordListPrompt(task),
  })
}
