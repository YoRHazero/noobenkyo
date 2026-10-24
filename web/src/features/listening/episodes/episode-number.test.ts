import { describe, expect, it } from 'vitest'
import { parseEpisodeNumber } from '@/features/listening/episodes/episode-number'

describe('parseEpisodeNumber', () => {
  it('takes any finite number', () => {
    expect(['2', ' 2.5 ', '-1', '0'].map(parseEpisodeNumber)).toEqual([2, 2.5, -1, 0])
  })

  it('refuses the rest', () => {
    expect(['', 'a', 'Infinity', '1e999'].map(parseEpisodeNumber)).toEqual([null, null, null, null])
  })
})
