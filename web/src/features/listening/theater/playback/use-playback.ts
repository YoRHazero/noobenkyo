import { useEffect, useState, useSyncExternalStore } from 'react'
import type { CastMember, EpisodeSummary } from '@/bindings'
import { loadListeningAudio } from '@/backend/listening/load-listening-audio'
import { useListeningEpisode } from '@/backend/listening/use-listening-episode'
import { missingAudio } from '@/features/listening/theater/playback/missing-audio'
import { rememberedModes, rememberModes } from '@/features/listening/theater/playback/mode-memory'
import type { PlayMode } from '@/features/listening/theater/playback/play-mode'
import { Player } from '@/features/listening/theater/playback/player'
import { firstEpisode, nextEpisode } from '@/features/listening/theater/playback/episode-order'
import { castModes, type VoiceModes } from '@/features/listening/theater/playback/voice-modes'

type Input = {
  world: string
  cast: CastMember[]
  /** The world's episodes in order. */
  episodes: EpisodeSummary[]
  from?: string
}

/**
 * The theater's playback: a `Player` fed with the episode it asks for and
 * the modes chosen for the cast, which are remembered per world.
 */
export function usePlayback({ world, cast, episodes, from }: Input) {
  const [wanted, setWanted] = useState(() => firstEpisode(episodes, from))
  const [player] = useState(
    () =>
      new Player({
        load: loadListeningAudio,
        nextEpisode: (uuid) => nextEpisode(episodes, uuid),
        requestEpisode: setWanted,
      }),
  )
  const [modes, setModes] = useState<VoiceModes>(() => castModes(cast, rememberedModes(world)))
  const episode = useListeningEpisode(wanted ?? '')
  const snapshot = useSyncExternalStore(player.subscribe, player.getSnapshot)

  useEffect(() => player.setModes(modes), [player, modes])
  useEffect(() => {
    if (episode.data) {
      player.show(episode.data)
    }
  }, [player, episode.data])
  useEffect(() => () => player.detach(), [player])

  const shown = snapshot.episode
  return {
    snapshot,
    player,
    modes,
    /** Takes effect from the next line on. */
    setMode: (voice: string | null, mode: PlayMode) => {
      const next =
        voice === null
          ? { ...modes, other: mode }
          : { ...modes, cast: { ...modes.cast, [voice]: mode } }
      setModes(next)
      rememberModes(world, next)
    },
    missing: shown ? missingAudio(shown.lines, modes) : { ja: 0, zh: 0 },
    /** Whether the episode has lines whose voice is not in the cast. */
    otherVoices: shown?.lines.some((line) => !(line.voice in modes.cast)) ?? false,
    loadError: episode.error,
    noEpisode: wanted === null || episode.data === null,
  }
}
