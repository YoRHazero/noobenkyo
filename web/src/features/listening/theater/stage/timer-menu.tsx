import { useEffect, useState } from 'react'
import { Timer } from 'lucide-react'
import { cn } from '@/components/ui/cn'
import {
  clockText,
  remainingMs,
  timerChoices,
  timerName,
  type PlayClock,
  type TimerChoice,
} from '@/features/listening/theater/playback/play-timer'
import { StageButton } from '@/features/listening/theater/stage/stage-button'

type Props = {
  clock: PlayClock
  onChoose: (choice: TimerChoice) => void
}

/**
 * When playing stops by itself. A timed choice counts down while playing;
 * the countdown is only shown, the player checks the time itself.
 */
export function TimerMenu({ clock, onChoose }: Props) {
  const [open, setOpen] = useState(false)
  const [now, setNow] = useState(() => performance.now())
  const ticking = clock.since !== null && typeof clock.choice === 'number'
  useEffect(() => {
    if (!ticking) {
      return
    }
    const timer = setInterval(() => setNow(performance.now()), 1000)
    return () => clearInterval(timer)
  }, [ticking])

  // While paused the clock does not move, so any moment will do.
  const left = remainingMs(clock, ticking ? now : 0)
  return (
    <div className="relative">
      <StageButton label="定时停止" active={open} onClick={() => setOpen(!open)}>
        <Timer />
        <span className="tabular-nums">
          {left === null ? timerName(clock.choice) : clockText(left)}
        </span>
      </StageButton>
      {open && (
        <ul className="absolute top-full right-0 z-30 mt-1 w-32 rounded-md border border-white/10 bg-black/80 py-1 backdrop-blur">
          {timerChoices.map((choice) => (
            <li key={choice}>
              <button
                type="button"
                className={cn(
                  'w-full px-3 py-1.5 text-left text-sm text-white/60 hover:bg-white/10 hover:text-white',
                  choice === clock.choice && 'text-white',
                )}
                onMouseDown={(event) => event.preventDefault()}
                onClick={() => {
                  onChoose(choice)
                  setOpen(false)
                }}
              >
                {timerName(choice)}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
