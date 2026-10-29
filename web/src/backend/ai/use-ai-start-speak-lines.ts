import { useMutation } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { aiTaskStarted } from '@/backend/ai/ai-tasks'
import { unwrap } from '@/backend/unwrap'

/**
 * Starts speaking the lines the episode lacks audio for; resolves to the
 * task as soon as it runs. Nothing has changed yet, so no cached result is refetched: the
 * task's events do that as recordings are stored.
 */
export function useAiStartSpeakLines() {
  return useMutation({
    mutationFn: (episode: string) => unwrap(commands.aiStartSpeakLines(episode)),
    onSuccess: (task, episode) => aiTaskStarted(task, { kind: 'speak_lines', episode }),
  })
}
