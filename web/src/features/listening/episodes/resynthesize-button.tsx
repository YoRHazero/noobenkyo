import { useState } from 'react'
import { RefreshCw } from 'lucide-react'
import type { Language, LineView } from '@/bindings'
import { useSpeakLinesTask } from '@/backend/ai/ai-tasks'
import { useAiStartSpeakLines } from '@/backend/ai/use-ai-start-speak-lines'
import { useListeningClearAudio } from '@/backend/listening/use-listening-clear-audio'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { languageNames, languages } from '@/features/listening/language-names'

type Props = {
  line: LineView
  episode: string
}

/**
 * Synthesizes one language of a line again: drops its audio there, then
 * synthesizes what the episode lacks, which now includes it. Only the
 * languages the line has audio in are offered; the rest are synthesized by
 * the episode's 合成. Not while the episode is being synthesized.
 */
export function ResynthesizeButton({ line, episode }: Props) {
  const [open, setOpen] = useState(false)
  const task = useSpeakLinesTask(episode)
  const clear = useListeningClearAudio()
  const start = useAiStartSpeakLines()
  const busy = clear.isPending || start.isPending
  const running = task !== undefined && task.outcome === null
  const voiced = languages.filter((language) => line.audio.includes(language))

  const redo = async (language: Language) => {
    try {
      await clear.mutateAsync({ lines: [line.id], language })
      await start.mutateAsync(episode)
      setOpen(false)
    } catch {
      // The popover stays open with the error.
    }
  }

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="重新合成"
          disabled={running || voiced.length === 0}
        >
          <RefreshCw />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="end" className="flex w-auto flex-col gap-2 p-2">
        <div className="flex items-center gap-1">
          <span className="px-1 text-xs text-muted-foreground">重新合成</span>
          {voiced.map((language) => (
            <Button
              key={language}
              variant="outline"
              size="sm"
              disabled={busy}
              onClick={() => void redo(language)}
            >
              {languageNames[language]}
            </Button>
          ))}
        </div>
        <ErrorText error={clear.error ?? start.error} />
      </PopoverContent>
    </Popover>
  )
}
