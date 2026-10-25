import { useState } from 'react'
import type { ScriptTarget } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { ImportScriptDialog } from '@/features/listening/script/import-script-dialog'

type Props = {
  target: ScriptTarget
  label: string
  /** The dialog's title. */
  title: string
}

/** Opens the script preview for a pasted or opened script; closing it forgets the text. */
export function ImportScriptButton({ target, label, title }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm">
          {label}
        </Button>
      </DialogTrigger>
      {open && <ImportScriptDialog target={target} title={title} onClose={() => setOpen(false)} />}
    </Dialog>
  )
}
