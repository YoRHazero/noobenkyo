import type { EpisodeSummary } from '@/bindings'
import { episodeLabel } from '@/features/listening/episode-label'

type Props = {
  /** The episode last clicked in either box; `undefined` before any is. */
  episode: EpisodeSummary | undefined
}

/**
 * The title and summary of the episode last clicked, under the boxes. It
 * keeps its height whatever it shows, so the form does not jump; a long
 * summary scrolls inside it.
 */
export function ShownEpisode({ episode }: Props) {
  return (
    <div className="h-24 overflow-y-auto rounded-md bg-muted/50 px-3 py-2 text-sm">
      {episode && (
        <>
          <p className="font-medium">
            <span className="mr-2 text-muted-foreground tabular-nums">
              {episodeLabel(episode.number)}
            </span>
            {episode.title}
          </p>
          <p className="mt-1 whitespace-pre-wrap text-muted-foreground">{episode.summary}</p>
        </>
      )}
    </div>
  )
}
