import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { ImportDialog } from '@/features/dictionary/import/import-dialog'

/** Opens the import dialog; closing it forgets the text and options. */
export function ImportButton() {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>导入</Button>
      </DialogTrigger>
      {open && <ImportDialog onClose={() => setOpen(false)} />}
    </Dialog>
  )
}
