import type { ReadingKey } from '@/bindings'
import { Trash2 } from 'lucide-react'
import { useDictionaryDeleteReading } from '@/backend/dictionary/use-dictionary-delete-reading'
import { ConfirmButton } from '@/components/confirm-button'

/** Deletes a reading with nothing below it, on the second click. */
export function DeleteReadingButton({ reading }: { reading: ReadingKey }) {
  const deleteReading = useDictionaryDeleteReading()
  return (
    <ConfirmButton
      label={<Trash2 />}
      ariaLabel="删除读音"
      confirmLabel="确认删除"
      size="icon-sm"
      disabled={deleteReading.isPending}
      onConfirm={() => deleteReading.mutate(reading)}
    />
  )
}
