import { useQuery } from '@tanstack/react-query'
import { commands, type WordListTask } from '@/bindings'

/**
 * The prompt for writing a new word list. It is worked out as the form
 * changes, so a click can put it on the clipboard at once.
 */
export function useAgentWordListPrompt(task: WordListTask) {
  return useQuery({
    queryKey: ['agent_word_list_prompt', task],
    queryFn: () => commands.agentWordListPrompt(task),
  })
}
