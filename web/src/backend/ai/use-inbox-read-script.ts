import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** The script waiting under the name; `null` once it is saved or discarded. */
export function useInboxReadScript(name: string) {
  return useQuery({
    queryKey: ['inbox_read_script', name],
    queryFn: () => unwrap(commands.inboxReadScript(name)),
  })
}
