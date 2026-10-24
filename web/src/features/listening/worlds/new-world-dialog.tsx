import { useState } from 'react'
import { useListeningCreateWorld } from '@/backend/listening/use-listening-create-world'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogFooter, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'

type Props = {
  onCreated: (name: string) => void
}

/** Creates a world by name; the setting may be left for later. */
export function NewWorldDialog({ onCreated }: Props) {
  const [name, setName] = useState('')
  const [setting, setSetting] = useState('')
  const create = useListeningCreateWorld()
  const busy = create.isPending
  const keepOpen = (event: Event) => {
    if (busy) {
      event.preventDefault()
    }
  }
  return (
    <DialogContent
      aria-describedby={undefined}
      showCloseButton={!busy}
      onEscapeKeyDown={keepOpen}
      onInteractOutside={keepOpen}
    >
      <DialogTitle>新建世界</DialogTitle>
      <Input
        aria-label="名字"
        placeholder="名字"
        value={name}
        disabled={busy}
        onChange={(event) => setName(event.target.value)}
      />
      <Textarea
        aria-label="设定"
        placeholder="设定：时代、地点、氛围、规则"
        value={setting}
        disabled={busy}
        className="min-h-28"
        onChange={(event) => setSetting(event.target.value)}
      />
      <ErrorText error={create.error} />
      <DialogFooter>
        <Button
          disabled={busy}
          onClick={() =>
            create.mutate({ name, setting }, { onSuccess: () => onCreated(name.trim()) })
          }
        >
          新建
        </Button>
      </DialogFooter>
    </DialogContent>
  )
}
