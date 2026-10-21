import { Link } from '@tanstack/react-router'
import { useDictionaryStats } from '@/backend/dictionary/use-dictionary-stats'
import { ErrorText } from '@/components/error-text'

/**
 * Why there is no quiz. Filling the zone found no untouched sense with
 * sentences, so the counts tell which of three cases it is.
 */
export function NothingToLearn() {
  const stats = useDictionaryStats()
  if (stats.error) {
    return <ErrorText error={stats.error} />
  }
  if (!stats.data) {
    return null
  }
  const { words, with_sentence } = stats.data
  return (
    <div className="flex flex-col items-center gap-3 rounded-xl border bg-card py-12 text-center">
      {words === 0 ? (
        <Reason text="词库为空" to="/dictionary" link="去导入 →" />
      ) : with_sentence === 0 ? (
        <Reason text="没有带例句的义项" to="/dictionary" link="去导入 →" />
      ) : (
        <Reason text="没有待学习的义项" to="/" link="回首页 →" />
      )}
    </div>
  )
}

function Reason({ text, to, link }: { text: string; to: '/' | '/dictionary'; link: string }) {
  return (
    <>
      <p className="text-muted-foreground">{text}</p>
      <Link to={to} className="text-primary underline-offset-4 hover:underline">
        {link}
      </Link>
    </>
  )
}
