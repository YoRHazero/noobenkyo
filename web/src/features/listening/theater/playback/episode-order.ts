import type { EpisodeSummary } from '@/bindings'

/** The episode to start from: the one asked for if it is there, else the first. */
export function firstEpisode(episodes: readonly EpisodeSummary[], from?: string): string | null {
  const asked = episodes.find((episode) => episode.uuid === from)
  return (asked ?? episodes[0])?.uuid ?? null
}

/** The episode after this one in the world's order; `null` after the last. */
export function nextEpisode(episodes: readonly EpisodeSummary[], current: string): string | null {
  const index = episodes.findIndex((episode) => episode.uuid === current)
  return index === -1 ? null : (episodes[index + 1]?.uuid ?? null)
}
