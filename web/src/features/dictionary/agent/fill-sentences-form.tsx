import { useState } from 'react'
import type { ContentFilter } from '@/bindings'
import { useAiFillSentencesPrompt } from '@/backend/ai/use-ai-fill-sentences-prompt'
import { useDictionarySearch } from '@/backend/dictionary/use-dictionary-search'
import { CopyButton } from '@/components/copy-button'
import { ErrorText } from '@/components/error-text'
import { petFailed, petRequested } from '@/stores/pet'
import { CountField } from '@/features/dictionary/agent/count-field'
import { fillRangeText } from '@/features/dictionary/agent/fill-range-text'
import { parsePositiveInteger } from '@/features/dictionary/agent/positive-integer'

type Props = {
  /** The dictionary page's filter; its layers are not used. */
  filter: ContentFilter
}

/**
 * Filling in sentences for senses that have none, from the rows the page's
 * keyword and progress show. The agent exports them itself, in the page's
 * order, so the count here is what it will find.
 */
export function FillSentencesForm({ filter }: Props) {
  const [limit, setLimit] = useState('20')
  const [perSense, setPerSense] = useState('3')
  const rows = useDictionarySearch(
    { text: filter.text, completeness: ['NoSentence'], progress: filter.progress },
    0,
    1,
  )
  const limitValue = parsePositiveInteger(limit)
  const perSenseValue = parsePositiveInteger(perSense)
  const prompt = useAiFillSentencesPrompt({
    keyword: filter.text,
    progress: filter.progress,
    limit: limitValue ?? 0,
    per_sense: perSenseValue ?? 0,
  })
  const total = rows.data?.total
  const ready = limitValue !== null && perSenseValue !== null && total !== undefined && total > 0

  return (
    <div className="flex flex-col gap-4">
      <p className="text-sm text-muted-foreground">
        {total === undefined ? '范围：…' : fillRangeText(filter.text, filter.progress, total)}
      </p>
      <div className="flex flex-col gap-2">
        <CountField label="取几行" value={limit} onChange={setLimit} />
        <CountField label="每个义项几条" value={perSense} onChange={setPerSense} />
      </div>
      <div className="flex items-center justify-end gap-3">
        <div className="mr-auto">
          <ErrorText error={rows.error ?? prompt.error} />
        </div>
        <CopyButton
          text={ready ? prompt.data : undefined}
          label="复制给 agent"
          onCopied={petRequested}
          onFailed={petFailed}
        />
      </div>
    </div>
  )
}
