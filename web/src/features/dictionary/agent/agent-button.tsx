import { useState } from 'react'
import type { ContentFilter } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { AgentDialog } from '@/features/dictionary/agent/agent-dialog'

type Props = {
  /** The dictionary page's filter: filling in sentences starts from what it shows. */
  filter: ContentFilter
}

/** Opens the tasks for an agent; closing it forgets what was typed. */
export function AgentButton({ filter }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">交给 agent…</Button>
      </DialogTrigger>
      {open && <AgentDialog filter={filter} />}
    </Dialog>
  )
}
