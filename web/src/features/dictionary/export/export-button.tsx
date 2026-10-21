import { useState } from 'react'
import type { ContentFilter } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { ExportDialog } from '@/features/dictionary/export/export-dialog'

type Props = {
  /** The dictionary page's filter: what it shows is what is exported. */
  filter: ContentFilter
  /** Rows matching the filter. */
  total: number
}

export function ExportButton({ filter, total }: Props) {
  const [open, setOpen] = useState(false)
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">导出</Button>
      </DialogTrigger>
      {open && <ExportDialog filter={filter} total={total} />}
    </Dialog>
  )
}
