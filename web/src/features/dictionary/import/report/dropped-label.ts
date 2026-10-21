import type { Dropped } from '@/bindings'

/** A problem as a row status: a whole line skipped, or part of it left out. */
export function droppedLabel(dropped: Dropped): string {
  switch (dropped.kind) {
    case 'entry':
      return '跳过：非法'
    case 'sentence':
      return '警告：非法例句'
    case 'sense':
      return '警告：义项未合并'
  }
}
