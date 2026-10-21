import { describe, expect, it } from 'vitest'
import { preselectedRating } from '@/features/study/quiz/preselected-rating'

describe('preselectedRating', () => {
  it('follows the judgement, and leaves the rest to the user', () => {
    expect(preselectedRating('Correct')).toBe('Known')
    expect(preselectedRating('Wrong')).toBe('Unknown')
    expect(preselectedRating('Unsure')).toBeNull()
    expect(preselectedRating(null)).toBeNull()
  })
})
