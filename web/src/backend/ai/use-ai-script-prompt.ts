import { useQuery } from '@tanstack/react-query'
import { commands, type ScriptTask } from '@/bindings'

/**
 * The prompt for writing a new episode or continuing one. It is worked
 * out as the form changes, so a click can put it on the clipboard at once.
 */
export function useAiScriptPrompt(task: ScriptTask) {
  return useQuery({
    queryKey: ['ai_script_prompt', task],
    queryFn: () => commands.aiScriptPrompt(task),
  })
}
