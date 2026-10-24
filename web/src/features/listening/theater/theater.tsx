import { useEffect, useState } from 'react'
import type { CastMember, EpisodeSummary, Language } from '@/bindings'
import { Backdrop } from '@/features/listening/theater/backdrop/backdrop'
import { canPlay } from '@/features/listening/theater/playback/missing-audio'
import { usePlayback } from '@/features/listening/theater/playback/use-playback'
import { BacklogPanel } from '@/features/listening/theater/panels/backlog-panel'
import { ModesPanel } from '@/features/listening/theater/panels/modes-panel'
import { ChapterCard } from '@/features/listening/theater/stage/chapter-card'
import { ControlBar, type Panel } from '@/features/listening/theater/stage/control-bar'
import { DialogueBox } from '@/features/listening/theater/stage/dialogue-box'
import { TopBar } from '@/features/listening/theater/stage/top-bar'
import { allSubtitles, type Subtitles } from '@/features/listening/theater/subtitles'
import { theaterKey } from '@/features/listening/theater/theater-keys'

type Props = {
  world: string
  cast: CastMember[]
  episodes: EpisodeSummary[]
  from?: string
  onClose: () => void
}

/**
 * The stage: the backdrop, the line in its dialogue box, the controls, and a
 * panel over them. Everything here reads the player's snapshot and calls its
 * actions; only the player touches the sound.
 */
export function Theater({ world, cast, episodes, from, onClose }: Props) {
  const playback = usePlayback({ world, cast, episodes, from })
  const { snapshot, player } = playback
  const [panel, setPanel] = useState<Panel | null>(null)
  const [subtitles, setSubtitles] = useState<Subtitles>(allSubtitles)
  const lineKey = `${snapshot.episode?.uuid}:${snapshot.lineIndex}`
  const [revealedFor, setRevealedFor] = useState<string | null>(null)
  const revealed = revealedFor === lineKey

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      const target = event.target
      if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
        return
      }
      const action = theaterKey(event)
      if (action === null) {
        return
      }
      event.preventDefault()
      switch (action) {
        case 'toggle':
          return player.toggle()
        case 'next':
          return player.next()
        case 'previous':
          return player.previous()
        case 'replay':
          return player.replay()
        case 'auto':
          return player.setAuto(!player.getSnapshot().auto)
        case 'reveal':
          return setRevealedFor(lineKey)
        case 'close':
          return setPanel(null)
      }
    }
    window.addEventListener('keydown', onKey, true)
    return () => window.removeEventListener('keydown', onKey, true)
  }, [player, lineKey])

  const episode = snapshot.episode
  const line = episode && !snapshot.card ? (episode.lines[snapshot.lineIndex] ?? null) : null
  const togglePanel = (next: Panel) => setPanel(panel === next ? null : next)
  const toggleSubtitle = (language: Language) =>
    setSubtitles({ ...subtitles, [language]: !subtitles[language] })

  return (
    <div className="fixed inset-0 overflow-hidden bg-black text-white select-none">
      <Backdrop
        world={world}
        episode={episode?.uuid ?? null}
        line={line}
        playing={snapshot.playing}
      />
      <div className="relative z-10 flex h-full flex-col">
        <TopBar
          world={world}
          episode={episode}
          clock={snapshot.clock}
          onTimer={(choice) => player.setTimer(choice)}
          onClose={onClose}
        />
        <div className="flex flex-1 items-center justify-center px-6">
          {episode && (snapshot.card || snapshot.blocked) && (
            <ChapterCard
              key={episode.uuid}
              episode={episode}
              blocked={snapshot.blocked}
              onStart={() => player.toggle()}
            />
          )}
          {!episode && playback.noEpisode && <p className="text-white/50">无集</p>}
          {playback.loadError && <p className="text-destructive">{playback.loadError.message}</p>}
        </div>
        <div className="mx-auto flex w-full max-w-4xl flex-col gap-2 px-6">
          {snapshot.notice && <p className="text-center text-sm text-warning">{snapshot.notice}</p>}
          {line && !snapshot.blocked && (
            <DialogueBox
              line={line}
              speaking={snapshot.playing}
              subtitles={subtitles}
              revealed={revealed}
              onReveal={() => setRevealedFor(lineKey)}
            />
          )}
          <ControlBar
            playing={snapshot.playing}
            auto={snapshot.auto}
            subtitles={subtitles}
            panel={panel}
            modesWarning={!canPlay(playback.missing)}
            onPrevious={() => player.previous()}
            onReplay={() => player.replay()}
            onToggle={() => player.toggle()}
            onNext={() => player.next()}
            onAuto={() => player.setAuto(!snapshot.auto)}
            onSubtitle={toggleSubtitle}
            onPanel={togglePanel}
          />
        </div>
      </div>
      {panel === 'backlog' && episode && (
        <BacklogPanel
          episode={episode}
          lineIndex={snapshot.lineIndex}
          onJump={(index) => player.jumpTo(index)}
          onClose={() => setPanel(null)}
        />
      )}
      {panel === 'modes' && (
        <ModesPanel
          cast={cast}
          modes={playback.modes}
          missing={playback.missing}
          otherVoices={playback.otherVoices}
          onMode={playback.setMode}
          onClose={() => setPanel(null)}
        />
      )}
    </div>
  )
}
