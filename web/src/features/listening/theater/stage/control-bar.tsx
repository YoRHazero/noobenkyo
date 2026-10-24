import { ChevronLeft, ChevronsRight, List, Pause, Play, RotateCcw, Settings2 } from 'lucide-react'
import type { Language } from '@/bindings'
import { languages } from '@/features/listening/language-names'
import type { Subtitles } from '@/features/listening/theater/subtitles'
import { StageButton } from '@/features/listening/theater/stage/stage-button'

export type Panel = 'backlog' | 'modes'

type Props = {
  playing: boolean
  auto: boolean
  subtitles: Subtitles
  panel: Panel | null
  /** Something about the modes needs a look: audio is missing. */
  modesWarning: boolean
  onPrevious: () => void
  onReplay: () => void
  onToggle: () => void
  onNext: () => void
  onAuto: () => void
  onSubtitle: (language: Language) => void
  onPanel: (panel: Panel) => void
}

const subtitleNames: Record<Language, string> = { ja: '日', zh: '中' }

/** The row of faint buttons under the dialogue box; each also has a key. */
export function ControlBar(props: Props) {
  return (
    <div className="flex items-center justify-center gap-1 py-3">
      <StageButton label="上一句（←）" onClick={props.onPrevious}>
        <ChevronLeft />
      </StageButton>
      <StageButton label="重听这一句（R）" onClick={props.onReplay}>
        <RotateCcw />
      </StageButton>
      <StageButton label={props.playing ? '暂停（空格）' : '继续（空格）'} onClick={props.onToggle}>
        {props.playing ? <Pause /> : <Play />}
      </StageButton>
      <StageButton label="下一句（→）" onClick={props.onNext}>
        <ChevronsRight />
      </StageButton>
      <StageButton label="自动（A）" active={props.auto} onClick={props.onAuto}>
        自动
      </StageButton>
      <span className="mx-2 h-4 w-px bg-white/15" />
      {languages.map((language) => (
        <StageButton
          key={language}
          label={`${subtitleNames[language]}字幕`}
          active={props.subtitles[language]}
          onClick={() => props.onSubtitle(language)}
        >
          {subtitleNames[language]}
        </StageButton>
      ))}
      <span className="mx-2 h-4 w-px bg-white/15" />
      <StageButton
        label="回看"
        active={props.panel === 'backlog'}
        onClick={() => props.onPanel('backlog')}
      >
        <List />
      </StageButton>
      <StageButton
        label="模式"
        active={props.panel === 'modes'}
        className={props.modesWarning ? 'text-warning' : undefined}
        onClick={() => props.onPanel('modes')}
      >
        <Settings2 />
      </StageButton>
    </div>
  )
}
