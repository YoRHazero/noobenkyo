import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

export function useStudySettings() {
  return useQuery({
    queryKey: ['study_settings'],
    queryFn: () => unwrap(commands.studySettings()),
  })
}
