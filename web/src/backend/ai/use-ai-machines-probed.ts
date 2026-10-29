import { useEffect } from 'react'
import { useQueryClient } from '@tanstack/react-query'
import { events } from '@/bindings'

/** Reads the machine reports again whenever every machine has been probed. */
export function useAiMachinesProbed() {
  const queryClient = useQueryClient()
  useEffect(() => {
    const stopping = events.aiMachinesProbed.listen(() =>
      queryClient.invalidateQueries({ queryKey: ['ai_machine_reports'] }),
    )
    return () => {
      void stopping.then((stop) => stop())
    }
  }, [queryClient])
}
