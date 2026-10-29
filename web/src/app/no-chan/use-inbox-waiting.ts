import { useInboxChanged } from '@/backend/inbox/use-inbox-changed'
import { useInboxList } from '@/backend/inbox/use-inbox-list'

/**
 * Whether no-chan reads her phone: something waits in the inbox, or the
 * inbox cannot be read, which her phone then says. Follows the inbox's
 * changes, so this holds whether the phone is open or not.
 */
export function useInboxWaiting(): boolean {
  useInboxChanged()
  const items = useInboxList()
  return items.isError || (items.data?.length ?? 0) > 0
}
