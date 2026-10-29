import { Link, useNavigate } from '@tanstack/react-router'
import { ChevronLeft, Play } from 'lucide-react'
import { useListeningEpisode } from '@/backend/listening/use-listening-episode'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { episodeLabel } from '@/features/listening/episode-label'
import { GenerateButton } from '@/features/listening/generate/generate-button'
import { ImportScriptButton } from '@/features/listening/script/import-script-button'
import { DeleteEpisodeButton } from '@/features/listening/episodes/delete-episode-button'
import { EpisodeHeader } from '@/features/listening/episodes/episode-header'
import { LineList } from '@/features/listening/episodes/line-list'
import { SynthesisControl } from '@/features/listening/episodes/synthesis-control'

type Props = {
  uuid: string
}

/**
 * An episode to edit: its header and its lines. Playing it opens the theater
 * there; synthesizing fills the audio it lacks.
 */
export function EpisodePage({ uuid }: Props) {
  const episode = useListeningEpisode(uuid)
  const navigate = useNavigate()

  if (episode.error) {
    return <ErrorText error={episode.error} />
  }
  if (episode.data === undefined) {
    return <p className="text-sm text-muted-foreground">加载中…</p>
  }
  if (episode.data === null) {
    return <p className="text-sm text-muted-foreground">无</p>
  }
  const detail = episode.data
  return (
    <div className="flex flex-col gap-6">
      <Link
        to="/listening/worlds/$name"
        params={{ name: detail.world }}
        className="flex items-center gap-1 self-start text-sm text-muted-foreground hover:text-foreground"
      >
        <ChevronLeft className="size-4" />
        {detail.world}
      </Link>
      <EpisodeHeader key={detail.uuid} episode={detail} />
      <div className="flex flex-wrap items-center gap-2">
        <Button size="sm" asChild>
          <Link
            to="/listening/worlds/$name/play"
            params={{ name: detail.world }}
            search={{ from: detail.uuid }}
          >
            <Play />
            播放
          </Link>
        </Button>
        <ImportScriptButton
          target={{ kind: 'append', episode: detail.uuid }}
          label="续写：导入…"
          title={`续写：${detail.title}`}
        />
        <GenerateButton
          world={detail.world}
          appending={detail.uuid}
          label="续写：生成…"
          title={`续写：${episodeLabel(detail.number)} ${detail.title}`}
        />
        <SynthesisControl episode={detail.uuid} />
        <div className="ml-auto">
          <DeleteEpisodeButton
            episode={detail.uuid}
            onDeleted={() =>
              navigate({ to: '/listening/worlds/$name', params: { name: detail.world } })
            }
          />
        </div>
      </div>
      <LineList episode={detail.uuid} lines={detail.lines} />
    </div>
  )
}
