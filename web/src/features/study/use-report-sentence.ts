import { commands, type Zone } from '@/bindings'
import { unwrap } from '@/components/result'
import { useEndQuestion } from '@/features/study/use-end-question'

export function useReportSentence(zone: Zone, onEnded: () => void) {
  return useEndQuestion(() => unwrap(commands.reportSentence(zone)), onEnded)
}
