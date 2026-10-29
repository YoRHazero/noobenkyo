import { useState } from 'react'
import { useMutation } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import type { ScriptTarget } from '@/bindings'
import { useListeningAppendScript } from '@/backend/listening/use-listening-append-script'
import { useListeningImportScript } from '@/backend/listening/use-listening-import-script'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogTitle } from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { openScriptFile } from '@/features/listening/script/open-script-file'
import { ScriptPreview } from '@/features/listening/script/script-preview'
import { useScriptEditor } from '@/features/listening/script/use-script-editor'
import { useSettledText } from '@/features/listening/script/use-settled-text'

type Props = {
  target: ScriptTarget
  /** The title shown for the target: 「导入剧本：星见町的新一集」. */
  title: string
  onClose: () => void
}

/**
 * A script the user pasted or opened, shown in the script preview. An edit
 * made in the preview replaces the text in the box with the edited script,
 * so the box is locked while an editor is open. Saving a new episode goes to
 * its page; continuing one closes the dialog.
 */
export function ImportScriptDialog({ target, title, onClose }: Props) {
  const [text, setText] = useState('')
  const { settled, settle } = useSettledText(text)
  const editor = useScriptEditor()
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
  const boxLocked = busy || editor.status !== 'closed'

  // The preview has checked the edited text already.
  const edit = async (next: string) => {
    setText(next)
    settle(next)
    return []
  }

  const save = async (script: string) => {
    if (target.kind === 'newEpisode') {
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
              disabled={boxLocked || read.isPending}
              onClick={() => read.mutate()}
            >
              打开
            </Button>
          </div>
          <Textarea
            aria-label="剧本"
            placeholder='{ "title": "…", "summary": "…", "lines": [ { "speaker": "…", "ja": "…", "zh": "…" } ] }'
            spellCheck={false}
            value={text}
            disabled={boxLocked}
            className="min-h-0 flex-1 resize-none font-mono text-xs field-sizing-fixed"
            onChange={(event) => setText(event.target.value)}
          />
        </div>
        <ScriptPreview
          target={target}
          text={settled}
          stale={settled !== text}
          editor={editor}
          edit={edit}
          save={save}
          discard={async () => onClose()}
        />
      </div>
    </DialogContent>
  )
}
