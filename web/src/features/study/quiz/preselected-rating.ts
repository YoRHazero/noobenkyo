import type { Judgement, Rating } from '@/bindings'

/**
 * The rating offered after submitting: right → 会, wrong → 不会. When the
 * rule cannot tell, or the question takes no input, the user must choose.
 */
export function preselectedRating(judgement: Judgement | null): Rating | null {
  switch (judgement) {
    case 'Correct':
      return 'Known'
    case 'Wrong':
      return 'Unknown'
    default:
      return null
  }
}
