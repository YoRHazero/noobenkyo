import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

export function useAiSettings() {
  return useQuery({
    queryKey: ['ai_settings'],
    queryFn: () => commands.aiSettings(),
  })
}
