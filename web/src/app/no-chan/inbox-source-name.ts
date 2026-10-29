import type { InboxSource } from '@/bindings'

/** Who sent an inbox item, as the list shows it. */
export const inboxSourceName: Record<InboxSource, string> = {
  agent: 'agent',
  ai: 'AI',
}
