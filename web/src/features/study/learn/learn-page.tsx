import { useLearnNext } from '@/backend/learn/use-learn-next'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { LearnQuiz } from '@/features/study/learn/learn-quiz'
import { NothingToLearn } from '@/features/study/learn/nothing-to-learn'

/**
 * Learning, one quiz after another. A new quiz arrives after every confirm,
 * skip or report; keying on when it arrived starts each one afresh, even when
 * the same sense comes again.
 */
export function LearnPage() {
  const next = useLearnNext()
  return (
    <Page title="学习">
      <ErrorText error={next.error} />
      {next.isPending && <p className="text-sm text-muted-foreground">加载中…</p>}
      {next.data === null && <NothingToLearn />}
      {next.data && <LearnQuiz key={next.dataUpdatedAt} quiz={next.data} />}
    </Page>
  )
}
