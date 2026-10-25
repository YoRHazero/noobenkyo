import { useState } from 'react'
import type { InboxItem } from '@/bindings'
import { useInboxChanged } from '@/backend/ai/use-inbox-changed'
import { useInboxList } from '@/backend/ai/use-inbox-list'
import { useInboxTakeEntries } from '@/backend/ai/use-inbox-take-entries'
import { useDictionaryGeneratedImportOptions } from '@/backend/dictionary/use-dictionary-generated-import-options'
import { cn } from '@/components/ui/cn'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { InboxList } from '@/app/inbox/inbox-list'
import { OpenedItem, type Opened } from '@/app/inbox/opened-item'

type Props = {
  /** The navigation's own look, so the menu reads as one of its items. */
  className: string
}

/**
 * 「inbox」 in the navigation, with how many items wait. It never opens by
 * itself; clicked, it lists them. Opening one closes the list: entries are
 * taken out first and go to the import dialog with the options for
 * generated lists; a script opens its preview and stays until it is saved
 * or discarded.
 */
export function InboxMenu({ className }: Props) {
  useInboxChanged()
  const items = useInboxList()
  const options = useDictionaryGeneratedImportOptions()
  const take = useInboxTakeEntries()
  const [listOpen, setListOpen] = useState(false)
  const [opened, setOpened] = useState<Opened | null>(null)
  const count = items.data?.length ?? 0

  const open = async (item: InboxItem) => {
    if (item.kind === 'script') {
      setListOpen(false)
      setOpened({ kind: 'script', name: item.name })
      return
    }
    if (!options.data) {
      return
    }
    const text = await take.mutateAsync(item.name).catch(() => null)
    if (text !== null) {
      setListOpen(false)
      setOpened({ kind: 'entries', text, options: options.data })
    }
  }

  return (
    <>
      <Popover
        open={listOpen}
        onOpenChange={(open) => {
          setListOpen(open)
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
