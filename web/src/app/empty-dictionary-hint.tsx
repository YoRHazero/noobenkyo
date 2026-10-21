import { Link } from '@tanstack/react-router'
import type { DictionaryStats } from '@/bindings'

type Props = {
  stats: DictionaryStats
}

/**
 * Why both counts are zero, and where to go. Every sense with sentences is
 * either untouched or at some stage, so this shows exactly when both are zero.
 */
export function EmptyDictionaryHint({ stats }: Props) {
  if (stats.with_sentence > 0) {
    return null
  }
  return (
    <p className="text-sm text-muted-foreground">
      {stats.words === 0 ? '词库为空' : '没有带例句的义项'}
      <Link to="/dictionary" className="ml-3 text-primary underline-offset-4 hover:underline">
        去导入 →
      </Link>
    </p>
  )
}
