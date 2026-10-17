import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { useDictionaryStats } from '@/features/dictionary/use-dictionary-stats'

/** Counts of the dictionary. */
export function DictionaryOverview() {
  const stats = useDictionaryStats()
  const items = stats.data
    ? [
        { label: '单词', value: stats.data.words },
        { label: '义项', value: stats.data.senses },
        { label: '有例句的义项', value: stats.data.senses_with_sentences },
        { label: '没学过的义项', value: stats.data.unlearned_senses },
      ]
    : []
  return (
    <Section title="词库">
      <ErrorText error={stats.error} />
      <dl className="grid grid-cols-4 gap-4">
        {items.map((item) => (
          <div key={item.label}>
            <dt className="text-sm text-muted-foreground">{item.label}</dt>
            <dd className="text-2xl font-semibold tabular-nums">{item.value}</dd>
          </div>
        ))}
      </dl>
    </Section>
  )
}
