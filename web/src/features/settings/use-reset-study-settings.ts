import { useMutation, useQueryClient } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/components/result'
import { settingsKeys } from '@/features/settings/settings-keys'

export function useResetStudySettings() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: () => unwrap(commands.resetStudySettings()),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: settingsKeys.study }),
  })
}
