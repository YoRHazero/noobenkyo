import type { ProgressState } from '@/bindings'
import { progressStateName } from '@/features/dictionary/progress-state-names'

/**
 * Which rows filling in sentences starts from: the page's keyword and
 * progress, always senses without sentences. 「范围：关键词「食」、进度：未学习、没例句（37 行）」
 */
export function fillRangeText(
  keyword: string | null,
  progress: ProgressState,
  rows: number,
): string {
  const parts: string[] = []
  if (keyword !== null) {
    parts.push(`关键词「${keyword}」`)
  }
  if (progress !== 'Any') {
    parts.push(`进度：${progressStateName[progress]}`)
  }
  parts.push('没例句')
  return `范围：${parts.join('、')}（${rows} 行）`
}
