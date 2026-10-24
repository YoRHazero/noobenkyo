import type { EpisodeDetail } from '@/bindings'
import { episodeLabel } from '@/features/listening/episode-label'

type Props = {
  episode: EpisodeDetail
  /** The webview would not start sound by itself: a click starts it. */
  blocked: boolean
  onStart: () => void
}

/** An episode's title in the middle of the stage, before its first line. */
export function ChapterCard({ episode, blocked, onStart }: Props) {
  return (
    <div className="flex flex-col items-center gap-3 text-white motion-safe:animate-in motion-safe:fade-in motion-safe:duration-700">
      <span className="text-sm tracking-[0.3em] text-white/50 tabular-nums">
        {episodeLabel(episode.number)}
      </span>
      <span className="text-3xl font-semibold tracking-wide">{episode.title}</span>
      {blocked && (
        <button
          type="button"
          className="mt-6 rounded-full border border-white/30 px-6 py-2 text-sm text-white/80 hover:bg-white/10"
          onMouseDown={(event) => event.preventDefault()}
          onClick={onStart}
        >
          点击开始
        </button>
      )}
    </div>
  )
}
