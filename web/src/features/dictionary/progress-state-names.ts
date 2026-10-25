import type { ProgressState } from '@/bindings'

/** How a progress condition is named on the dictionary page. */
export const progressStateName: Record<ProgressState, string> = {
  Any: '不限',
  Untouched: '未学习',
  Touched: '已学习',
}
