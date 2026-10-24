import { describe, expect, it } from 'vitest'
import type { EpisodeSummary } from '@/bindings'
import { firstEpisode, nextEpisode } from '@/features/listening/theater/playback/episode-order'

const episode = (uuid: string): EpisodeSummary => ({
  uuid,
  number: 1,
  title: '',
  summary: '',
  created_at: '',
  lines: 1,
  char_count: 1,
  missing_ja: 0,
  missing_zh: 0,
})
const episodes = [episode('a'), episode('b')]

describe('episode order', () => {
  it('starts where asked, or at the first episode', () => {
    expect(firstEpisode(episodes, 'b')).toBe('b')
    expect(firstEpisode(episodes, 'gone')).toBe('a')
    expect(firstEpisode([], undefined)).toBeNull()
  })

  it('goes on in the world’s order and stops after the last', () => {
    expect(nextEpisode(episodes, 'a')).toBe('b')
    expect(nextEpisode(episodes, 'b')).toBeNull()
  })
})
