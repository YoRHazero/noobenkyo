import type { ImportOptions } from '@/bindings'
import { Dialog } from '@/components/ui/dialog'
import { ImportDialog } from '@/features/dictionary/import/import-dialog'
import { InboxScriptDialog } from '@/features/listening/script/inbox-script-dialog'

/**
 * The one item open from the inbox. Entries have left the inbox by now, so
 * their text comes along; a script stays there and is opened by its name.
 */
export type Opened =
  { kind: 'entries'; text: string; options: ImportOptions } | { kind: 'script'; name: string }

type Props = {
  opened: Opened | null
  onClose: () => void
}

/** Hands an inbox item to the feature it belongs to: the import dialog, or the script preview. */
export function OpenedItem({ opened, onClose }: Props) {
  return (
    <Dialog open={opened !== null} onOpenChange={(open) => !open && onClose()}>
      {opened?.kind === 'entries' && (
        <ImportDialog initialText={opened.text} initialOptions={opened.options} onClose={onClose} />
      )}
      {opened?.kind === 'script' && <InboxScriptDialog name={opened.name} onClose={onClose} />}
    </Dialog>
  )
}
