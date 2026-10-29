import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

/** The defaults never change while the app runs. */
export function useAiDefaultSettings() {
  return useQuery({
    queryKey: ['ai_default_settings'],
    queryFn: () => commands.aiDefaultSettings(),
    staleTime: Infinity,
  })
}
