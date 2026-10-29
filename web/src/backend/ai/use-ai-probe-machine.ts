import { useMutation } from '@tanstack/react-query'
import { commands, type Machine } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/**
 * Probes one machine as the form has it, saved or not. Nothing is kept or
 * refetched: the report is the caller's to show. A machine that saving
 * would refuse is refused with the same reason.
 */
export function useAiProbeMachine() {
  return useMutation({
    mutationFn: (machine: Machine) => unwrap(commands.aiProbeMachine(machine)),
  })
}
