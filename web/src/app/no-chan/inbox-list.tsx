import type { InboxItem } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { ScriptTargetText } from '@/features/listening/script/script-target-text'
import { inboxSourceName } from '@/app/no-chan/inbox-source-name'

type Props = {
  /** Oldest first; `undefined` while loading. */
  items: InboxItem[] | undefined
  error: Error | null
  /** Opening an item is on its way, or cannot start yet. */
  busy: boolean
  onOpen: (item: InboxItem) => void
}

/** What was delivered, in the order it arrived: each by who sent it, its name and where it goes. */
export function InboxList({ items, error, busy, onOpen }: Props) {
  return (
    <div className="flex flex-col gap-1">
      {!items ? (
        <p className="px-2 py-1 text-sm text-muted-foreground">加载中…</p>
      ) : items.length === 0 ? (
        <p className="px-2 py-1 text-sm text-muted-foreground">无</p>
      ) : (
        <ul className="flex flex-col">
          {items.map((item) => (
            <li key={`${item.source}/${item.name}`}>
              <button
                type="button"
                disabled={busy}
                className="w-full rounded-md px-2 py-1.5 text-left text-sm hover:bg-accent disabled:pointer-events-none disabled:opacity-50"
                onClick={() => onOpen(item)}
              >
                <span className="mr-2 rounded bg-muted px-1 text-xs text-muted-foreground">
                  {inboxSourceName[item.source]}
                </span>
                <span className="font-medium">{item.name}</span>
                <span className="ml-2 text-muted-foreground">
                  {item.kind === 'entries' ? (
                    '条目'
                  ) : (
                    <>
                      剧本：
                      <ScriptTargetText target={item.target} />
                    </>
                  )}
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
      <div className="px-2">
        <ErrorText error={error} />
      </div>
    </div>
  )
}
