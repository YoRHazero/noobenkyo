import { commands, type AiSettings } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

/** Refused while a model call runs or waits, or when the settings are invalid. */
export function useAiUpdateSettings() {
  return useChange((settings: AiSettings) => unwrap(commands.aiUpdateSettings(settings)))
}
