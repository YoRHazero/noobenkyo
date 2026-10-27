import { useQuery } from '@tanstack/react-query'
import { commands, type InboxSource } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** The script waiting from the source under the name; `null` once it is saved or discarded. */
export function useInboxReadScript(source: InboxSource, name: string) {
  return useQuery({
    queryKey: ['inbox_read_script', source, name],
    queryFn: () => unwrap(commands.inboxReadScript(source, name)),
  })
}
