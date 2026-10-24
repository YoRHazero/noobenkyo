import { useEffect, useRef } from 'react'
import { Play } from 'lucide-react'
import type { EpisodeDetail } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { episodeLabel } from '@/features/listening/episode-label'
import { RubyLine } from '@/features/listening/ruby-line'
import { SidePanel } from '@/features/listening/theater/panels/side-panel'
import { StageButton } from '@/features/listening/theater/stage/stage-button'

type Props = {
  episode: EpisodeDetail
  lineIndex: number
  onJump: (lineIndex: number) => void
  onClose: () => void
}

/**
 * Every line of the episode on stage, in full whatever the subtitles say.
 * ▶ plays from a line; the rest of the row is text to select.
 */
export function BacklogPanel({ episode, lineIndex, onJump, onClose }: Props) {
  const current = useRef<HTMLLIElement>(null)
  useEffect(() => {
    current.current?.scrollIntoView({ block: 'nearest' })
  }, [lineIndex, episode.uuid])

  return (
    <SidePanel title={`回看 · ${episodeLabel(episode.number)} ${episode.title}`} onClose={onClose}>
      <ol className="flex flex-col gap-1 select-text">
        {episode.lines.map((line, index) => (
          <li
            key={line.id}
            ref={index === lineIndex ? current : undefined}
            className={cn(
              'grid grid-cols-[2.25rem_1.5rem_4.5rem_1fr] items-start gap-x-2 rounded-lg px-1 py-2',
              index === lineIndex && 'bg-white/10',
            )}
          >
            <StageButton label={`从第 ${index + 1} 行开始放`} onClick={() => onJump(index)}>
              <Play />
            </StageButton>
            <span className="pt-2 text-right text-xs text-white/40 tabular-nums">{index + 1}</span>
            <span className="truncate pt-1.5 text-sm text-white/60">{line.speaker}</span>
            <div className="flex flex-col">
              <RubyLine
                segments={line.ja_ruby}
                className="text-base text-white [&_rt]:text-white/55"
              />
              <span className="text-sm text-white/55">{line.zh}</span>
            </div>
          </li>
        ))}
      </ol>
    </SidePanel>
  )
}
