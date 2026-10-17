import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { settingsKeys } from '@/features/settings/settings-keys'

export function useStudySettings() {
  return useQuery({
    queryKey: settingsKeys.study,
    queryFn: () => unwrap(commands.studySettings()),
  })
}
