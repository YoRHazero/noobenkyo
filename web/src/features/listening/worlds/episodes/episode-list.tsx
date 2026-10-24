import { Link } from '@tanstack/react-router'
import { Play } from 'lucide-react'
import type { EpisodeSummary } from '@/bindings'
import { Button } from '@/components/ui/button'
import { episodeLabel } from '@/features/listening/episode-label'
import { missingAudioText } from '@/features/listening/missing-audio-text'

type Props = {
  world: string
  /** In their order, which is also the order the theater goes on in. */
  episodes: EpisodeSummary[]
}

/** The world's episodes: ▶ starts the theater there, the rest opens the episode. */
export function EpisodeList({ world, episodes }: Props) {
  if (episodes.length === 0) {
    return <p className="text-sm text-muted-foreground">无</p>
  }
  return (
    <ul className="flex flex-col gap-1.5">
      {episodes.map((episode) => {
        const missing = missingAudioText(episode.missing_ja, episode.missing_zh)
        return (
          <li
            key={episode.uuid}
            className="flex items-start gap-2 rounded-lg border bg-card px-2 py-2"
          >
            <Button variant="ghost" size="icon-sm" aria-label="播放" asChild>
              <Link
                to="/listening/worlds/$name/play"
                params={{ name: world }}
                search={{ from: episode.uuid }}
              >
                <Play />
              </Link>
            </Button>
            <Link
              to="/listening/episodes/$uuid"
              params={{ uuid: episode.uuid }}
              className="flex min-w-0 flex-1 flex-col gap-0.5 rounded-md px-1 py-1 hover:bg-accent"
            >
              <span className="text-sm">
                <span className="mr-2 text-muted-foreground tabular-nums">
                  {episodeLabel(episode.number)}
                </span>
                <span className="font-medium">{episode.title}</span>
              </span>
              <span className="truncate text-xs text-muted-foreground">{episode.summary}</span>
              <span className="text-xs text-muted-foreground">
                行：{episode.lines}
                {missing && <span className="ml-2 text-warning">{missing}</span>}
              </span>
            </Link>
          </li>
        )
      })}
    </ul>
  )
}
