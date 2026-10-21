import { senseState, senseStateLabel } from '@/features/dictionary/sense-state'
import { timeText } from '@/features/dictionary/time-text'

type Progress = {
  stage: number
  passes: number
  lapses: number
  modified_at: string | null
}

/** "学习中（3/7）；正确：2；错误：1；最后修改：2026-10-20 21:05" */
export function progressText(progress: Progress, maxStage: number): string {
  const state = senseStateLabel[senseState(progress.stage, progress.passes, maxStage)]
  const parts = [
    `${state}（${progress.stage}/${maxStage}）`,
    `正确：${progress.passes}`,
    `错误：${progress.lapses}`,
  ]
  if (progress.modified_at) {
    parts.push(`最后修改：${timeText(progress.modified_at)}`)
  }
  return parts.join('；')
}
