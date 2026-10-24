import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { NewWorldDialog } from '@/features/listening/worlds/new-world-dialog'

type Props = {
  onCreated: (name: string) => void
}

export function NewWorldButton({ onCreated }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>新建世界</Button>
      </DialogTrigger>
      {open && (
        <NewWorldDialog
          onCreated={(name) => {
            setOpen(false)
            onCreated(name)
          }}
        />
      )}
    </Dialog>
  )
}
