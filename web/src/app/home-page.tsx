import { useDictionaryStats } from '@/backend/dictionary/use-dictionary-stats'
import { EmptyDictionaryHint } from '@/app/empty-dictionary-hint'
import { HomeEntrance } from '@/app/home-entrance'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'

/**
 * The entrances to learning and review, with their counts. The entrances are
 * always open: whether there is a quiz is for those pages to find out.
 */
export function HomePage() {
  const stats = useDictionaryStats()
  const data = stats.data
  const toReview = data?.stages.reduce((sum, stage) => sum + stage.senses, 0)
  return (
    <Page title="首页">
      <div className="grid grid-cols-2 gap-4">
        <HomeEntrance to="/learn" title="学习" label="待学习" count={data?.untouched} />
        <HomeEntrance to="/review" title="复习" label="待复习" count={toReview} />
      </div>
      {data && <EmptyDictionaryHint stats={data} />}
      <ErrorText error={stats.error} />
    </Page>
  )
}
