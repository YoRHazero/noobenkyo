import { useMutation, useQueryClient } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Probes every machine in the settings again; its reports replace the last ones. */
export function useAiProbeMachines() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: () => unwrap(commands.aiProbeMachines()),
    onSuccess: (reports) => queryClient.setQueryData(['ai_machine_reports'], reports),
  })
}
