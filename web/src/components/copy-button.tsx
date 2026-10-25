import { useState } from 'react'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { writeClipboard } from '@/components/write-clipboard'

type Props = {
  /** What goes on the clipboard; `undefined` while it is not ready, which greys the button. */
  text: string | undefined
  label: string
  /** After a copy, with where the button was when clicked. */
  onCopied?: (from: DOMRect) => void
  onFailed?: () => void
}

/**
 * Puts a ready text on the clipboard and says so next to the button, until
 * the text changes. The text must be at hand before the click: the webview
 * only lets a page write the clipboard while handling the click.
 */
export function CopyButton({ text, label, onCopied, onFailed }: Props) {
  const [copied, setCopied] = useState<string | null>(null)
  const [error, setError] = useState<Error | null>(null)

  const copy = (event: React.MouseEvent<HTMLButtonElement>) => {
    if (text === undefined) {
      return
    }
    const from = event.currentTarget.getBoundingClientRect()
    setError(null)
    writeClipboard(text).then(
      () => {
        setCopied(text)
        onCopied?.(from)
      },
      (error: Error) => {
        setError(error)
        onFailed?.()
      },
    )
  }

  return (
    <div className="flex items-center gap-3">
      {error ? (
        <ErrorText error={error} />
      ) : (
        copied !== null &&
        copied === text && <span className="text-sm text-muted-foreground">已复制</span>
      )}
      <Button disabled={text === undefined} onClick={copy}>
        {label}
      </Button>
    </div>
  )
}
