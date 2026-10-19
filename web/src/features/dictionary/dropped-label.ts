import type { Dropped } from '@/bindings'

/** What an import problem took out, in words. */
export function droppedLabel(dropped: Dropped): string {
  switch (dropped.kind) {
    case 'entry':
      return '整条未导入'
    case 'sentence':
      return '丢掉例句'
    case 'sense':
      return '义项未合并'
  }
}
