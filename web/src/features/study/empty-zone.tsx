import type { ZoneStatus } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { useStartReviewGroup } from '@/features/study/use-start-review-group'

type Props = {
  status: ZoneStatus
  onStarted: () => void
}

/** What an empty zone says, and starting a new group in fixed-group mode. */
export function EmptyZone({ status, onStarted }: Props) {
  const start = useStartReviewGroup(onStarted)
  return (
    <Card>
      <CardContent className="flex flex-col items-center gap-4 py-10 text-center">
        <p className="text-muted-foreground">{message(status)}</p>
        {status.group_finished && (
          <Button disabled={start.isPending} onClick={() => start.mutate(undefined)}>
            开始新一组
          </Button>
        )}
        <ErrorText error={start.error} />
      </CardContent>
    </Card>
  )
}

function message(status: ZoneStatus): string {
  if (status.zone === 'Learning') {
    return status.new_today >= status.daily_new_limit
      ? '今天的新词数已经到上限了。'
      : '词库里没有待学的新词。导入带例句的词条后再来。'
  }
  return status.group_finished ? '这一组复习完了。' : '没有可以复习的义项。'
}
