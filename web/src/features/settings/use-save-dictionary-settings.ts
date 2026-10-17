import { useMutation, useQueryClient } from '@tanstack/react-query'
import { commands, type DictionarySettings } from '@/bindings'
import { unwrap } from '@/components/result'
import { settingsKeys } from '@/features/settings/settings-keys'

export function useSaveDictionarySettings() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (settings: DictionarySettings) =>
      unwrap(commands.updateDictionarySettings(settings)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: settingsKeys.dictionary }),
  })
}
