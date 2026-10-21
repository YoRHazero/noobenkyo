import type { ComponentProps } from 'react'
import type { ImportPreview } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { PreviewResult } from '@/features/dictionary/import/preview/preview-result'

type Props = Omit<ComponentProps<typeof PreviewResult>, 'preview'> & {
  /** `undefined` until the first preview arrives. */
  preview: ImportPreview | undefined
  error: Error | null
}

/** The table view of the text under the current options. */
export function PreviewView({ preview, error, ...rest }: Props) {
  if (error) {
    return <ErrorText error={error} />
  }
  if (!preview) {
    return <p className="text-sm text-muted-foreground">加载中…</p>
  }
  return <PreviewResult preview={preview} {...rest} />
}
