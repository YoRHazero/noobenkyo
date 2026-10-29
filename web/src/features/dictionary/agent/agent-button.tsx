import { useState } from 'react'
import { NooChanButton, NooChanDialog } from 'no-chan'
import type { ContentFilter } from '@/bindings'
import { AgentScreen } from '@/features/dictionary/agent/agent-screen'

type Props = {
  /** The dictionary page's filter: filling in sentences starts from what it shows. */
  filter: ContentFilter
}

/**
 * no-chan peeking over a button, which opens the tasks for an agent on her
 * phone. Closing the phone forgets what was typed.
 */
export function AgentButton({ filter }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <>
      <NooChanButton label="交给 agent" onClick={() => setOpen(true)} />
      <NooChanDialog open={open} onOpenChange={setOpen}>
        <AgentScreen filter={filter} />
      </NooChanDialog>
    </>
  )
}
