import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import type { InboxScript } from '@/bindings'
import { useInboxDiscardScript } from '@/backend/agent/use-inbox-discard-script'
import { useInboxReadScript } from '@/backend/agent/use-inbox-read-script'
import { useInboxReplaceScript } from '@/backend/agent/use-inbox-replace-script'
import { useInboxSaveScript } from '@/backend/agent/use-inbox-save-script'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogTitle } from '@/components/ui/dialog'
import { petFailed } from '@/stores/pet'
import { ScriptPreview } from '@/features/listening/script/script-preview'
import { ScriptTargetText } from '@/features/listening/script/script-target-text'
import { useScriptEditor } from '@/features/listening/script/use-script-editor'

type Props = {
  /** The name it was sent under. */
  name: string
  onClose: () => void
}

/**
 * A script an agent sent, in the script preview. Each edit is written back
 * to the inbox, where the agent reads it with `show`; saving stores it and
 * removes it, discarding just removes it. Closing keeps it in the inbox.
 */
export function InboxScriptDialog({ name, onClose }: Props) {
  const read = useInboxReadScript(name)
  /** The script as first read; the preview stays once it has opened, even if the script leaves. */
  const [opened, setOpened] = useState<InboxScript | null>(null)
  if (read.data && opened === null) {
    setOpened(read.data)
  }
  return (
    <DialogContent aria-describedby={undefined} className="flex h-[85vh] flex-col sm:max-w-3xl">
      {opened ? (
        <InboxScriptPreview
          name={name}
          opened={opened}
          latest={read.data ?? null}
          onClose={onClose}
        />
      ) : (
        <>
          <DialogTitle>{name} 剧本</DialogTitle>
          {read.error ? (
            <ErrorText error={read.error} />
          ) : (
            <p className="text-sm text-muted-foreground">
              {read.data === null ? 'inbox 里已经没有这一份' : '加载中…'}
            </p>
          )}
        </>
      )}
    </DialogContent>
  )
}

type PreviewProps = {
  name: string
  opened: InboxScript
  /** The script as the inbox has it now; `null` once it has left. */
  latest: InboxScript | null
  onClose: () => void
}

/**
 * Holds the text last loaded or written back. A different text read from
 * the inbox is the agent's new version: it replaces the one shown, unless an
 * open editor has changes, in which case the user chooses.
 */
function InboxScriptPreview({ name, opened, latest, onClose }: PreviewProps) {
  const [script, setScript] = useState(opened)
  /** The last text read from the inbox, so each new one is looked at once. */
  const [seen, setSeen] = useState(script.text)
  /** The agent's version the user chose not to load. */
  const [declined, setDeclined] = useState<string | null>(null)
  /** The agent's version waiting for the user's choice. */
  const [newer, setNewer] = useState<InboxScript | null>(null)
  const [reloaded, setReloaded] = useState(false)
  /** Writing back reads the inbox again, possibly before the write returns. */
  const [writing, setWriting] = useState(false)
  const editor = useScriptEditor()
  const navigate = useNavigate()
  const replace = useInboxReplaceScript()
  const saveScript = useInboxSaveScript()
  const discardScript = useInboxDiscardScript()
  // Saving or discarding removes the script before the dialog closes.
  const leaving = [saveScript, discardScript].some((change) => change.isPending || change.isSuccess)

  const load = (version: InboxScript) => {
    setScript(version)
    editor.setEditing(null)
    setNewer(null)
  }

  if (latest && !writing && latest.text !== seen) {
    setSeen(latest.text)
    if (latest.text !== script.text && latest.text !== declined) {
      if (editor.status === 'changed') {
        setNewer(latest)
      } else {
        load(latest)
        setReloaded(true)
      }
    }
  }
  const gone = latest === null && !leaving

  const edit = async (text: string) => {
    setWriting(true)
    try {
      const problems = await replace.mutateAsync(
        { name, target: script.target, text },
        { onError: petFailed },
      )
      if (problems.length === 0) {
        setScript({ target: script.target, text })
        // Written back over the agent's version: the user kept their own.
        setNewer(null)
        setReloaded(false)
      }
      return problems
    } finally {
      setWriting(false)
    }
  }

  const save = async (text: string) => {
    const uuid = await saveScript.mutateAsync(
      { name, target: script.target, text },
      { onError: petFailed },
    )
    onClose()
    if (script.target.kind === 'newEpisode') {
      await navigate({ to: '/listening/episodes/$uuid', params: { uuid } })
    }
  }

  const discard = async () => {
    await discardScript.mutateAsync(name, { onError: petFailed })
    onClose()
  }

  const notice = gone ? (
    <p className="text-sm text-warning">inbox 里已经没有这一份</p>
  ) : newer ? (
    <div className="flex flex-wrap items-center gap-2 rounded-md border border-warning px-3 py-2 text-sm">
      <span className="mr-auto">agent 交来了新版</span>
      <Button
        variant="outline"
        size="sm"
        onClick={() => {
          setDeclined(newer.text)
          setNewer(null)
        }}
      >
        保留手上这版
      </Button>
      <Button size="sm" onClick={() => load(newer)}>
        加载新版
      </Button>
    </div>
  ) : reloaded ? (
    <p className="text-sm text-muted-foreground">已加载 agent 交来的新版</p>
  ) : null

  return (
    <>
      <DialogTitle>
        {name} 剧本：
        <ScriptTargetText target={script.target} />
      </DialogTitle>
      <ScriptPreview
        target={script.target}
        text={script.text}
        locked={gone}
        editor={editor}
        edit={edit}
        save={save}
        discard={discard}
        notice={notice}
      />
    </>
  )
}
