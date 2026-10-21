import { useDictionaryDefaultImportOptions } from '@/backend/dictionary/use-dictionary-default-import-options'
import { useDictionaryImport } from '@/backend/dictionary/use-dictionary-import'
import { ErrorText } from '@/components/error-text'
import { DialogContent, DialogTitle } from '@/components/ui/dialog'
import { ImportEditor } from '@/features/dictionary/import/import-editor'
import { useImportDraft } from '@/features/dictionary/import/use-import-draft'
import { ImportReportView } from '@/features/dictionary/import/report/import-report-view'

type Props = {
  onClose: () => void
}

/**
 * The import dialog: the editor until the import is done, then its report.
 * While importing, the dialog cannot be closed. Escape closes it, except in
 * a field marked `data-own-escape` (the find bar), which uses Escape itself.
 */
export function ImportDialog({ onClose }: Props) {
  const draft = useImportDraft()
  const defaults = useDictionaryDefaultImportOptions()
  const importText = useDictionaryImport()
  const busy = importText.isPending
  const keepOpen = (event: Event) => {
    if (busy) {
      event.preventDefault()
    }
  }
  const keepOpenOnEscape = (event: KeyboardEvent) => {
    if (event.target instanceof Element && event.target.closest('[data-own-escape]')) {
      event.preventDefault()
    }
    keepOpen(event)
  }
  const options = draft.options ?? defaults.data

  return (
    <DialogContent
      aria-describedby={undefined}
      showCloseButton={!busy}
      onEscapeKeyDown={keepOpenOnEscape}
      onInteractOutside={keepOpen}
      className="flex h-[85vh] flex-col sm:max-w-5xl"
    >
      {importText.data ? (
        <ImportReportView
          report={importText.data}
          onBack={() => importText.reset()}
          onClose={onClose}
        />
      ) : options ? (
        <ImportEditor
          draft={draft}
          options={options}
          importing={busy}
          error={importText.error}
          onImport={() => importText.mutate({ text: draft.text, options })}
        />
      ) : (
        <>
          <DialogTitle>导入</DialogTitle>
          <ErrorText error={defaults.error} />
        </>
      )}
    </DialogContent>
  )
}
