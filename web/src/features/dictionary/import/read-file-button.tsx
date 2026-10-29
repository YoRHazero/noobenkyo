import { useMutation } from '@tanstack/react-query'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { openEntryFile } from '@/features/dictionary/entry-file/open-entry-file'

type Props = {
  disabled: boolean
  /** Receives the file's text, which replaces the text box. */
  onRead: (text: string) => void
}

/** Reads an entry file into the text box; cancelling changes nothing. */
export function ReadFileButton({ disabled, onRead }: Props) {
  const read = useMutation({
    mutationFn: openEntryFile,
    onSuccess: (text) => {
      if (text !== null) {
        onRead(text)
      }
    },
  })
  return (
    <div className="flex items-center gap-3">
      <ErrorText error={read.error} />
      <Button variant="outline" disabled={disabled || read.isPending} onClick={() => read.mutate()}>
        打开
      </Button>
    </div>
  )
}
