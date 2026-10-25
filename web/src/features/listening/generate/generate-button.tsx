import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { GenerateDialog } from '@/features/listening/generate/generate-dialog'

type Props = {
  world: string
  /** The episode to continue; `null` for a new one. */
  appending: string | null
  label: string
  /** The dialog's title. */
  title: string
}

/** Opens the generation form; closing it forgets what was chosen. */
export function GenerateButton({ world, appending, label, title }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm">
          {label}
        </Button>
      </DialogTrigger>
      {open && <GenerateDialog world={world} appending={appending} title={title} />}
    </Dialog>
  )
}
