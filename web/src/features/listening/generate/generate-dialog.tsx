import { useState } from 'react'
import { useAiScriptPrompt } from '@/backend/ai/use-ai-script-prompt'
import { useListeningEpisodes } from '@/backend/listening/use-listening-episodes'
import { CopyButton } from '@/components/copy-button'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Textarea } from '@/components/ui/textarea'
import { petFailed, petRequested } from '@/stores/pet'
import { PreviousEpisodes } from '@/features/listening/generate/previous-episodes'

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
  const prompt = useAiScriptPrompt({ world, appending, previous: chosen, premise })

  return (
    <DialogContent aria-describedby={undefined} className="sm:max-w-2xl">
      <DialogHeader>
        <DialogTitle>{title}</DialogTitle>
      </DialogHeader>
      <section className="flex flex-col gap-2">
        <h3 className="text-sm font-medium">前序集</h3>
        {episodes.error ? (
          <ErrorText error={episodes.error} />
        ) : !episodes.data ? (
          <p className="text-sm text-muted-foreground">加载中…</p>
        ) : (
          <PreviousEpisodes episodes={candidates} chosen={chosen} onChange={setPrevious} />
        )}
      </section>
      <section className="flex flex-col gap-2">
        <h3 className="text-sm font-medium">引子</h3>
        <Textarea
          aria-label="引子"
          placeholder="空着：交给 agent 判断"
          value={premise}
          className="min-h-24"
          onChange={(event) => setPremise(event.target.value)}
        />
      </section>
      <DialogFooter className="items-center">
        <div className="mr-auto">
          <ErrorText error={prompt.error} />
        </div>
        <Button variant="outline" disabled>
          生成
        </Button>
        <CopyButton
          text={prompt.data}
          label="复制给 agent"
          onCopied={petRequested}
          onFailed={petFailed}
        />
      </DialogFooter>
    </DialogContent>
  )
}
