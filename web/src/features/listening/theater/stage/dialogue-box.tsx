import type { LineView } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { RubyLine } from '@/features/listening/ruby-line'
import { nameHue } from '@/features/listening/name-hue'
import { narratorSpeaker, type Subtitles } from '@/features/listening/theater/subtitles'
import { SubtitleRow } from '@/features/listening/theater/stage/subtitle-row'

type Props = {
  line: LineView
  /** Sound is coming out: the name plate's dots move. */
  speaking: boolean
  subtitles: Subtitles
  /** The hidden languages were revealed for this line. */
  revealed: boolean
  onReveal: () => void
}

/**
 * The line on stage, as in a visual novel: a name plate in the voice's color
 * (none for the narrator, whose box is darker and cooler), the Japanese with
 * ruby, the Chinese under it. The text can be selected and copied.
 */
export function DialogueBox({ line, speaking, subtitles, revealed, onReveal }: Props) {
  const narrator = line.speaker === narratorSpeaker
  return (
    <div className="relative">
      {!narrator && (
        // Above the box: its frosted glass is a layer of its own, which would cover the plate.
        <div className="absolute -top-4 left-6 z-10 flex items-center gap-2">
          <span
            className="rounded-md px-3 py-1 text-sm font-medium text-white shadow-lg"
            style={{ backgroundColor: `oklch(0.5 0.12 ${nameHue(line.voice)})` }}
          >
            {line.speaker}
          </span>
          <SpeakingDots speaking={speaking} />
        </div>
      )}
      <div
        className={cn(
          'flex min-h-36 flex-col justify-center rounded-2xl border border-white/15 px-8 pt-7 pb-5 shadow-2xl backdrop-blur-md select-text',
          narrator ? 'bg-slate-950/70' : 'bg-black/45',
        )}
      >
        <div
          key={line.id}
          className="flex flex-col gap-1 motion-safe:animate-in motion-safe:fade-in motion-safe:duration-150"
        >
          <SubtitleRow shown={subtitles.ja || revealed} label="日语" onReveal={onReveal}>
            <RubyLine
              segments={line.ja_ruby}
              className="text-[28px] text-white [&_rt]:text-white/60"
            />
          </SubtitleRow>
          <SubtitleRow shown={subtitles.zh || revealed} label="中文" onReveal={onReveal}>
            <p className="text-lg text-white/70">{line.zh}</p>
          </SubtitleRow>
        </div>
        {narrator && (
          <div className="mt-1 self-end">
            <SpeakingDots speaking={speaking} />
          </div>
        )}
      </div>
    </div>
  )
}

function SpeakingDots({ speaking }: { speaking: boolean }) {
  return (
    <span className="flex gap-1" aria-hidden>
      {[0, 150, 300].map((delay) => (
        <span
          key={delay}
          className={cn(
            'size-1.5 rounded-full bg-white/60',
            speaking && 'motion-safe:animate-bounce',
          )}
          style={{ animationDelay: `${delay}ms` }}
        />
      ))}
    </span>
  )
}
