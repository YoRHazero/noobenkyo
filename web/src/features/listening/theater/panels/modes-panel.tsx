import type { CastMember, Language } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { missingAudioText } from '@/features/listening/missing-audio-text'
import {
  playModeNames,
  playModes,
  startMode,
  type PlayMode,
} from '@/features/listening/theater/playback/play-mode'
import type { VoiceModes } from '@/features/listening/theater/playback/voice-modes'
import { SidePanel } from '@/features/listening/theater/panels/side-panel'
import { roleNames } from '@/features/listening/worlds/cast/role-kinds'

type Props = {
  cast: CastMember[]
  modes: VoiceModes
  /** Lines of the episode on stage lacking audio their mode needs. */
  missing: Record<Language, number>
  /** The episode has voices outside the cast, which use the other mode. */
  otherVoices: boolean
  onMode: (voice: string | null, mode: PlayMode) => void
  onClose: () => void
}

/**
 * The mode of every voice in the cast, any of the four whatever references
 * the character has: missing audio is what stops playing. A change applies
 * from the next line on.
 */
export function ModesPanel({ cast, modes, missing, otherVoices, onMode, onClose }: Props) {
  const warning = missingAudioText(missing.ja, missing.zh)
  return (
    <SidePanel title="模式" onClose={onClose}>
      <div className="flex flex-col gap-3">
        {warning && <p className="text-sm text-warning">{warning}</p>}
        {cast.map((member) => (
          <ModeRow
            key={member.character}
            name={member.character}
            note={roleNames[member.role.kind]}
            mode={modes.cast[member.character] ?? startMode}
            onMode={(mode) => onMode(member.character, mode)}
          />
        ))}
        {otherVoices && (
          <ModeRow
            name="其他声音"
            note="不在可出场角色里"
            mode={modes.other}
            onMode={(mode) => onMode(null, mode)}
          />
        )}
      </div>
    </SidePanel>
  )
}

type RowProps = {
  name: string
  note: string
  mode: PlayMode
  onMode: (mode: PlayMode) => void
}

function ModeRow({ name, note, mode, onMode }: RowProps) {
  return (
    <div className="flex flex-col gap-1.5">
      <span className="text-sm">
        {name}
        <span className="ml-2 text-xs text-white/40">{note}</span>
      </span>
      <div className="flex gap-1">
        {playModes.map((option) => (
          <button
            key={option}
            type="button"
            aria-pressed={option === mode}
            className={cn(
              'rounded-md px-2.5 py-1 text-xs text-white/50 transition-colors hover:bg-white/10 hover:text-white',
              option === mode && 'bg-white/15 text-white',
            )}
            onMouseDown={(event) => event.preventDefault()}
            onClick={() => onMode(option)}
          >
            {playModeNames[option]}
          </button>
        ))}
      </div>
    </div>
  )
}
