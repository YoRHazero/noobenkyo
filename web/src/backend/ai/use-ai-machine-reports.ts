import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'

/**
 * What the last probe of every machine in the settings found. Empty until
 * the first, which starts with the app, ends; `useAiMachinesProbed` reads it
 * again then.
 */
export function useAiMachineReports() {
  return useQuery({
    queryKey: ['ai_machine_reports'],
    queryFn: () => commands.aiMachineReports(),
  })
}
