import { useState } from 'react'
import { NooChan, NooChanDialog } from 'no-chan'
import { nooChan } from '@/stores/no-chan'
import { InboxScreen } from '@/app/no-chan/inbox-screen'
import { OpenedItem, type Opened } from '@/app/no-chan/opened-item'
import { useInboxWaiting } from '@/app/no-chan/use-inbox-waiting'

/**
 * no-chan at the bottom of the pages with the navigation, and the inbox on
 * her phone. While something waits in it she reads her phone; clicked then,
 * she opens it. Opening an item closes the phone first, since the phone
 * sits above every other dialog.
 */
export function AppNooChan() {
  const waiting = useInboxWaiting()
  const [phoneOpen, setPhoneOpen] = useState(false)
  const [opened, setOpened] = useState<Opened | null>(null)

  return (
    <>
      <NooChan controller={nooChan} hasNotice={waiting} onClick={() => setPhoneOpen(true)} />
      <NooChanDialog open={phoneOpen} onOpenChange={setPhoneOpen}>
        <InboxScreen
          onOpened={(item) => {
            setPhoneOpen(false)
            setOpened(item)
          }}
        />
      </NooChanDialog>
      <OpenedItem opened={opened} onClose={() => setOpened(null)} />
    </>
  )
}
