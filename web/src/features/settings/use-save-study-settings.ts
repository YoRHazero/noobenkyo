import { useMutation, useQueryClient } from '@tanstack/react-query'
import { commands, type StudySettings } from '@/bindings'
import { unwrap } from '@/components/result'
import { settingsKeys } from '@/features/settings/settings-keys'

/** Replaces the study settings; the backend rejects invalid ones. */
export function useSaveStudySettings() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (settings: StudySettings) => unwrap(commands.updateStudySettings(settings)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: settingsKeys.study }),
  })
}
