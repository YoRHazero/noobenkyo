import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { NewCharacterDialog } from '@/features/listening/characters/new-character-dialog'

type Props = {
  onCreated: (name: string) => void
}

/** Opens the new-character form; closing it forgets what was typed. */
export function NewCharacterButton({ onCreated }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>新建角色</Button>
      </DialogTrigger>
      {open && (
        <NewCharacterDialog
          onCreated={(name) => {
            setOpen(false)
            onCreated(name)
          }}
        />
      )}
    </Dialog>
  )
}
