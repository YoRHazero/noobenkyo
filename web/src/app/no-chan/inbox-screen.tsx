import type { InboxItem } from '@/bindings'
import { useInboxList } from '@/backend/inbox/use-inbox-list'
import { useInboxTakeEntries } from '@/backend/inbox/use-inbox-take-entries'
import { useDictionaryGeneratedImportOptions } from '@/backend/dictionary/use-dictionary-generated-import-options'
import { nooChan } from '@/stores/no-chan'
import { InboxList } from '@/app/no-chan/inbox-list'
import type { Opened } from '@/app/no-chan/opened-item'

type Props = {
  /** An item is ready to be opened; the phone closes to make way for it. */
  onOpened: (opened: Opened) => void
}

/**
 * The inbox, as her phone's screen shows it. Entries are taken out of the
 * inbox before they are opened, with the options for generated lists; a
 * script is opened where it is. Closing the phone forgets a failed take.
 */
export function InboxScreen({ onOpened }: Props) {
  const items = useInboxList()
  const options = useDictionaryGeneratedImportOptions()
  const take = useInboxTakeEntries()

  const open = async (item: InboxItem) => {
    if (item.kind === 'script') {
      onOpened({ kind: 'script', source: item.source, name: item.name })
      return
    }
    if (!options.data) {
      return
    }
    const text = await take
      .mutateAsync({ source: item.source, name: item.name }, { onError: nooChan.notifyError })
      .catch(() => null)
    if (text !== null) {
      onOpened({ kind: 'entries', text, options: options.data })
    }
  }

  return (
    <div className="flex flex-col gap-3">
      <h2 className="px-2 text-sm font-semibold">inbox</h2>
      <InboxList
        items={items.data}
        error={items.error ?? options.error ?? take.error}
        busy={take.isPending || !options.data}
        onOpen={(item) => void open(item)}
      />
    </div>
  )
}
