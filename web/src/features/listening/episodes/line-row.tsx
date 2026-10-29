import { useState, type MouseEvent } from 'react'
import { Pencil } from 'lucide-react'
import type { LineView } from '@/bindings'
import { loadListeningAudio } from '@/backend/listening/load-listening-audio'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { ClipButton } from '@/features/listening/clip/clip-button'
import { languageNames, languages } from '@/features/listening/language-names'
import { RubyLine } from '@/features/listening/ruby-line'
import { LineEditor } from '@/features/listening/episodes/line-editor'
import { ResynthesizeButton } from '@/features/listening/episodes/resynthesize-button'

type Props = {
  episode: string
  line: LineView
  /** From 1, in the episode's order. */
  number: number
  ticked: boolean
  onTick: (shift: boolean) => void
}

/**
 * A line with its audio to listen to and to synthesize again. The voice is
 * named when it is not the speaker, since it decides who reads the line.
 */
export function LineRow({ episode, line, number, ticked, onTick }: Props) {
  const [editing, setEditing] = useState(false)
  return (
    <li className="grid grid-cols-[1.25rem_2rem_6rem_1fr_auto] items-start gap-x-3 py-2.5">
      <Checkbox
        aria-label={`选择第 ${number} 行`}
        checked={ticked}
        className="mt-1.5"
        onClick={(event: MouseEvent) => {
          event.preventDefault()
          onTick(event.shiftKey)
        }}
      />
      <span className="pt-1 text-right text-sm text-muted-foreground tabular-nums">{number}</span>
      <div className="flex flex-col pt-1 text-sm">
        <span className="truncate font-medium">{line.speaker}</span>
        {line.voice !== line.speaker && (
          <span className="truncate text-xs text-muted-foreground">声音：{line.voice}</span>
        )}
      </div>
      {editing ? (
        <LineEditor line={line} onDone={() => setEditing(false)} />
      ) : (
        <div className="flex flex-col">
          <RubyLine segments={line.ja_ruby} className="text-base" />
          <span className="text-sm text-muted-foreground">{line.zh}</span>
        </div>
      )}
      <div className="flex items-center">
        {languages.map((language) =>
          line.audio.includes(language) ? (
            <ClipButton
              key={language}
              clipKey={`line:${line.id}:${language}`}
              label={`试听${languageNames[language]}`}
              load={() => loadListeningAudio(line.id, language)}
            />
          ) : (
            <span key={language} className="w-8 text-center text-xs text-warning">
              缺{languageNames[language].slice(0, 1)}
            </span>
          ),
        )}
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="编辑"
          onClick={() => setEditing(!editing)}
        >
          <Pencil />
        </Button>
        <ResynthesizeButton line={line} episode={episode} />
      </div>
    </li>
  )
}
