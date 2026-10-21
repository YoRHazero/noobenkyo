import { Button } from '@/components/ui/button'
import type { ReviewMode } from '@/features/study/review/review-mode'

type Props = {
  mode: ReviewMode
  answered: number
  /** Whether the session asked anything at all. */
  hadAny: boolean
  starting: boolean
  onAgain: () => void
  onBack: () => void
}

/** The end of a session: nothing to review, a group done, or everything done. */
export function ReviewDone({ mode, answered, hadAny, starting, onAgain, onBack }: Props) {
  const text = !hadAny
    ? '没有待复习的义项'
    : mode === 'group'
      ? `本组完成；已答：${answered} 题`
      : `全部复习完；已答：${answered} 题`
  return (
    <div className="flex flex-col items-center gap-4 rounded-xl border bg-card p-10 shadow-sm">
      <p className="text-lg">{text}</p>
      <div className="flex gap-2">
        {hadAny && mode === 'group' && (
          <Button disabled={starting} onClick={onAgain}>
            再来一组
          </Button>
        )}
        <Button variant="outline" disabled={starting} onClick={onBack}>
          返回
        </Button>
      </div>
    </div>
  )
}
