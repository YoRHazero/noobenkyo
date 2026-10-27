import { useState } from 'react'
import type { InboxItem } from '@/bindings'
import { useInboxChanged } from '@/backend/inbox/use-inbox-changed'
import { useInboxList } from '@/backend/inbox/use-inbox-list'
import { useInboxTakeEntries } from '@/backend/inbox/use-inbox-take-entries'
import { useDictionaryGeneratedImportOptions } from '@/backend/dictionary/use-dictionary-generated-import-options'
import { cn } from '@/components/ui/cn'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { petFailed } from '@/stores/pet'
import { InboxList } from '@/app/inbox/inbox-list'
import { OpenedItem, type Opened } from '@/app/inbox/opened-item'

type Props = {
  /** The navigation's own look, so the menu reads as one of its items. */
  className: string
  /** Whether the list is open; the layout holds it, so the pet can open it too. */
  open: boolean
  onOpenChange: (open: boolean) => void
}

/**
 * 「inbox」 in the navigation, with how many items wait. It never opens by
 * itself; clicked (or the pet clicked while she reads her phone), it lists
 * them. Opening one closes the list: entries are
 * taken out first and go to the import dialog with the options for
 * generated lists; a script opens its preview and stays until it is saved
 * or discarded.
 */
export function InboxMenu({ className, open: listOpen, onOpenChange }: Props) {
  useInboxChanged()
  const items = useInboxList()
  const options = useDictionaryGeneratedImportOptions()
  const take = useInboxTakeEntries()
  const [opened, setOpened] = useState<Opened | null>(null)
  const count = items.data?.length ?? 0

  const open = async (item: InboxItem) => {
    if (item.kind === 'script') {
      onOpenChange(false)
      setOpened({ kind: 'script', source: item.source, name: item.name })
      return
    }
    if (!options.data) {
      return
    }
    const text = await take
      .mutateAsync({ source: item.source, name: item.name }, { onError: petFailed })
      .catch(() => null)
    if (text !== null) {
      onOpenChange(false)
      setOpened({ kind: 'entries', text, options: options.data })
    }
  }

  return (
    <>
      <Popover
        open={listOpen}
        onOpenChange={(open) => {
          onOpenChange(open)
          take.reset()
        }}
      >
        <PopoverTrigger asChild>
          <button type="button" className={cn(className, listOpen && 'bg-accent text-foreground')}>
            inbox
            {count > 0 && (
              <span className="ml-1 font-medium text-foreground tabular-nums">{count}</span>
            )}
          </button>
        </PopoverTrigger>
        <PopoverContent align="end" className="w-80 p-2">
          <InboxList
            items={items.data}
            error={items.error ?? options.error ?? take.error}
            busy={take.isPending || !options.data}
            onOpen={(item) => void open(item)}
          />
        </PopoverContent>
      </Popover>
      <OpenedItem opened={opened} onClose={() => setOpened(null)} />
    </>
  )
}
