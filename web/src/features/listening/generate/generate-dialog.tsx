import { useState } from 'react'
import { useListeningEpisodes } from '@/backend/listening/use-listening-episodes'
import { CopyButton } from '@/components/copy-button'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { nooChan } from '@/stores/no-chan'
import { DrawPremiseButton } from '@/features/listening/generate/draw-premise-button'
import { PreviousEpisodes } from '@/features/listening/generate/previous-episodes'
import { scriptPrompt } from '@/features/listening/generate/script-prompt'

type Props = {
  world: string
  /** The episode to continue; `null` for a new one. */
  appending: string | null
  /** 「生成：星见町的新一集」, 「续写：第 2 集 雨の日」. */
  title: string
}

/**
 * What a script is written from: the episodes before it, in order, and the
 * premise. Both may be left empty for the agent to judge. The starting
 * prompt is worked out as they change, so copying it is one click; whether
 * the world has its narrator and main characters is listening's to check
 * when the agent asks for the world. Generating here waits for the backend.
 */
export function GenerateDialog({ world, appending, title }: Props) {
  const episodes = useListeningEpisodes(world)
  const [previous, setPrevious] = useState<string[]>([])
  const [premise, setPremise] = useState('')
  // The episode continued is the agent's to read anyway; deleted ones drop out.
  const candidates = (episodes.data ?? []).filter((episode) => episode.uuid !== appending)
  const chosen = previous.filter((uuid) => candidates.some((episode) => episode.uuid === uuid))
  const prompt = scriptPrompt({ world, appending, previous: chosen, premise })

  return (
    <DialogContent aria-describedby={undefined} className="sm:max-w-2xl">
      <DialogHeader>
        <DialogTitle>{title}</DialogTitle>
      </DialogHeader>
      <section>
        {episodes.error ? (
          <ErrorText error={episodes.error} />
        ) : !episodes.data ? (
          <p className="text-sm text-muted-foreground">加载中…</p>
        ) : (
          <PreviousEpisodes episodes={candidates} chosen={chosen} onChange={setPrevious} />
        )}
      </section>
      <section className="flex flex-col gap-2">
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-medium">引子</h3>
          <DrawPremiseButton />
        </div>
        <Textarea
          aria-label="引子"
          placeholder="不指定"
          value={premise}
          className="min-h-24"
          onChange={(event) => setPremise(event.target.value)}
        />
      </section>
      <DialogFooter className="items-center">
        <Button variant="outline" disabled>
          生成
        </Button>
        <CopyButton
          text={prompt}
          label="复制 prompt"
          onCopied={nooChan.notifyRequest}
          onFailed={nooChan.notifyError}
        />
      </DialogFooter>
    </DialogContent>
  )
}
