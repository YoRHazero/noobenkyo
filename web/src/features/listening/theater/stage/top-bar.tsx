import { X } from 'lucide-react'
import type { EpisodeDetail } from '@/bindings'
import { episodeLabel } from '@/features/listening/episode-label'
import type { PlayClock, TimerChoice } from '@/features/listening/theater/playback/play-timer'
import { StageButton } from '@/features/listening/theater/stage/stage-button'
import { TimerMenu } from '@/features/listening/theater/stage/timer-menu'

type Props = {
  world: string
  episode: EpisodeDetail | null
  clock: PlayClock
  onTimer: (choice: TimerChoice) => void
  onClose: () => void
}

/** Where the theater is, the timer, and the way out; faint, so the line stays in front. */
export function TopBar({ world, episode, clock, onTimer, onClose }: Props) {
  return (
    <div className="flex items-center gap-3 px-6 py-3 text-sm text-white/45">
      <span>{world}</span>
      {episode && (
        <>
          <span>·</span>
          <span className="tabular-nums">{episodeLabel(episode.number)}</span>
          <span className="truncate">{episode.title}</span>
        </>
      )}
      <div className="ml-auto flex items-center gap-1">
        <TimerMenu clock={clock} onChoose={onTimer} />
        <StageButton label="退出" onClick={onClose}>
          <X />
        </StageButton>
      </div>
    </div>
  )
}
