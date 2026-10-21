import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

/** The defaults never change while the app runs. */
export function useStudyDefaultSettings() {
  return useQuery({
    queryKey: ['study_default_settings'],
    queryFn: () => commands.studyDefaultSettings(),
    staleTime: Infinity,
  })
}
