import { useState } from 'react'
import { useMutation } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { useListeningAppendScript } from '@/backend/listening/use-listening-append-script'
import { useListeningImportScript } from '@/backend/listening/use-listening-import-script'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogTitle } from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { openScriptFile } from '@/features/listening/script/open-script-file'
import { ScriptPreview } from '@/features/listening/script/script-preview'
import type { ScriptTarget } from '@/features/listening/script/script-target'

type Props = {
  target: ScriptTarget
  /** The title shown for the target: 「导入剧本：星见町的新一集」. */
  title: string
  onClose: () => void
}

/**
 * A script the user pasted or opened, shown in the script preview. Saving a
 * new episode goes to its page; continuing one closes the dialog.
 */
export function ImportScriptDialog({ target, title, onClose }: Props) {
  const [text, setText] = useState('')
  const navigate = useNavigate()
  const importScript = useListeningImportScript()
  const appendScript = useListeningAppendScript()
  const read = useMutation({
    mutationFn: openScriptFile,
    onSuccess: (read) => {
      if (read !== null) {
        setText(read)
      }
    },
  })
  const busy = importScript.isPending || appendScript.isPending
  const keepOpen = (event: Event) => {
    if (busy) {
      event.preventDefault()
    }
  }

  const save = async (script: string) => {
    if (target.kind === 'new') {
      const uuid = await importScript.mutateAsync({ world: target.world, text: script })
      onClose()
      await navigate({ to: '/listening/episodes/$uuid', params: { uuid } })
    } else {
      await appendScript.mutateAsync({ episode: target.episode, text: script })
      onClose()
    }
  }

  return (
    <DialogContent
      aria-describedby={undefined}
      showCloseButton={!busy}
      onEscapeKeyDown={keepOpen}
      onInteractOutside={keepOpen}
      className="flex h-[85vh] flex-col sm:max-w-5xl"
    >
      <DialogTitle>{title}</DialogTitle>
      <div className="grid min-h-0 flex-1 grid-cols-2 gap-4">
        <div className="flex min-h-0 flex-col gap-2">
          <div className="flex items-center justify-end gap-3">
            <ErrorText error={read.error} />
            <Button
              variant="outline"
              size="sm"
              disabled={busy || read.isPending}
              onClick={() => read.mutate()}
            >
              打开文件…
            </Button>
          </div>
          <Textarea
            aria-label="剧本"
            placeholder='{ "title": "…", "summary": "…", "lines": [ { "speaker": "…", "ja": "…", "zh": "…" } ] }'
            spellCheck={false}
            value={text}
            disabled={busy}
            className="min-h-0 flex-1 resize-none font-mono text-xs field-sizing-fixed"
            onChange={(event) => setText(event.target.value)}
          />
        </div>
        <ScriptPreview target={target} text={text} save={save} discard={onClose} />
      </div>
    </DialogContent>
  )
}
