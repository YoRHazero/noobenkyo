import { commands, type StudySettings } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export function useUpdateStudySettings() {
  return useChange((settings: StudySettings) => unwrap(commands.updateStudySettings(settings)))
}
