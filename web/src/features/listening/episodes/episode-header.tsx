import { useState } from 'react'
import { Pencil } from 'lucide-react'
import type { EpisodeDetail } from '@/bindings'
import { useListeningUpdateEpisode } from '@/backend/listening/use-listening-update-episode'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { episodeLabel } from '@/features/listening/episode-label'
import { missingAudioText } from '@/features/listening/missing-audio-text'
import { parseEpisodeNumber } from '@/features/listening/episodes/episode-number'

type Props = {
  episode: EpisodeDetail
}

/** The episode's number, title and summary, edited together in place. */
export function EpisodeHeader({ episode }: Props) {
  const [editing, setEditing] = useState(false)
  const missing = missingAudioText(
    episode.lines.filter((line) => !line.audio.includes('ja')).length,
    episode.lines.filter((line) => !line.audio.includes('zh')).length,
  )
  if (editing) {
    return <EpisodeHeaderEditor episode={episode} onDone={() => setEditing(false)} />
  }
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center gap-2">
        <h1 className="text-2xl font-semibold">
          <span className="mr-3 text-muted-foreground tabular-nums">
            {episodeLabel(episode.number)}
          </span>
          {episode.title}
        </h1>
        <Button variant="ghost" size="icon-sm" aria-label="编辑" onClick={() => setEditing(true)}>
          <Pencil />
        </Button>
      </div>
      <p className="text-sm text-muted-foreground">{episode.summary}</p>
      {missing && <p className="text-sm text-warning">{missing}</p>}
    </div>
  )
}

function EpisodeHeaderEditor({ episode, onDone }: Props & { onDone: () => void }) {
  const [number, setNumber] = useState(String(episode.number))
  const [title, setTitle] = useState(episode.title)
  const [summary, setSummary] = useState(episode.summary)
  const update = useListeningUpdateEpisode()
  const parsed = parseEpisodeNumber(number)
  const problems = update.data ?? []

  const save = async () => {
    if (parsed === null) {
      return
    }
    const rejected = await update
      .mutateAsync({ episode: episode.uuid, number: parsed, title, summary })
      .catch(() => null)
    if (rejected?.length === 0) {
      onDone()
    }
  }

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        <span className="text-sm text-muted-foreground">第</span>
        <Input
          aria-label="集数"
          inputMode="decimal"
          value={number}
          aria-invalid={parsed === null}
          disabled={update.isPending}
          className="h-8 w-20"
          onChange={(event) => setNumber(event.target.value)}
        />
        <span className="text-sm text-muted-foreground">集</span>
        <Input
          aria-label="标题"
          value={title}
          disabled={update.isPending}
          className="h-8"
          onChange={(event) => setTitle(event.target.value)}
        />
      </div>
      <Textarea
        aria-label="梗概"
        value={summary}
        disabled={update.isPending}
        onChange={(event) => setSummary(event.target.value)}
      />
      {problems.length > 0 && (
        <ul className="flex flex-col gap-1 text-sm text-destructive">
          {problems.map((problem) => (
            <li key={problem}>{problem}</li>
          ))}
        </ul>
      )}
      <div className="flex items-center justify-end gap-2">
        <ErrorText error={update.error} />
        <Button variant="outline" size="sm" disabled={update.isPending} onClick={onDone}>
          取消
        </Button>
        <Button size="sm" disabled={update.isPending || parsed === null} onClick={save}>
          保存
        </Button>
      </div>
    </div>
  )
}
