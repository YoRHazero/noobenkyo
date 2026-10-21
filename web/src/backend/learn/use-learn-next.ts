import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Asking again before answering gives the same quiz. */
export function useLearnNext() {
  return useQuery({
    queryKey: ['learn_next'],
    queryFn: () => unwrap(commands.learnNext()),
  })
}
