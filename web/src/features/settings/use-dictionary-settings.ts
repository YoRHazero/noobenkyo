import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { settingsKeys } from '@/features/settings/settings-keys'

export function useDictionarySettings() {
  return useQuery({
    queryKey: settingsKeys.dictionary,
    queryFn: () => commands.dictionarySettings(),
  })
}
