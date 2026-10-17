import { useMutation } from '@tanstack/react-query'
import { commands, type Zone } from '@/bindings'
import { unwrap } from '@/components/result'

export function useSubmitAnswer(zone: Zone) {
  return useMutation({
    mutationFn: (input: string) => unwrap(commands.submitAnswer(zone, input)),
  })
}
