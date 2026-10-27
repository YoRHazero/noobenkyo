import { useQuery } from '@tanstack/react-query'
import { commands } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Everything waiting in the inbox from every source, oldest first; read again on every change. */
export function useInboxList() {
  return useQuery({
    queryKey: ['inbox_list'],
    queryFn: () => unwrap(commands.inboxList()),
  })
}
