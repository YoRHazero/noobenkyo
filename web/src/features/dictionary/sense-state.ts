export type SenseState = 'untouched' | 'learning' | 'mastered'

/**
 * Where a sense stands, as Design.md defines it: untouched with no passes at
 * stage 0, mastered at stage N, learning otherwise.
 */
export function senseState(stage: number, passes: number, maxStage: number): SenseState {
  if (stage === 0 && passes === 0) {
    return 'untouched'
  }
  return stage >= maxStage ? 'mastered' : 'learning'
}

export const senseStateLabel: Record<SenseState, string> = {
  untouched: '未学习',
  learning: '学习中',
  mastered: '已掌握',
}
