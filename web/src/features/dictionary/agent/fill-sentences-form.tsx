import { useState } from 'react'
import type { ContentFilter } from '@/bindings'
import { useDictionarySearch } from '@/backend/dictionary/use-dictionary-search'
import { CopyButton } from '@/components/copy-button'
import { ErrorText } from '@/components/error-text'
import { nooChan } from '@/stores/no-chan'
import { CountField } from '@/features/dictionary/agent/count-field'
import { fillRangeText } from '@/features/dictionary/agent/fill-range-text'
import { fillSentencesPrompt } from '@/features/dictionary/agent/fill-sentences-prompt'
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
  const total = rows.data?.total
  const ready = limitValue !== null && perSenseValue !== null && total !== undefined && total > 0
  const prompt = ready
    ? fillSentencesPrompt({
        keyword: filter.text,
        progress: filter.progress,
        limit: limitValue,
        perSense: perSenseValue,
      })
    : undefined

  return (
    <div className="flex flex-col gap-4">
      <p className="text-sm text-muted-foreground">
        {total === undefined ? '范围：…' : fillRangeText(filter.text, filter.progress, total)}
      </p>
      <div className="flex flex-col gap-2">
        <CountField label="行数" value={limit} onChange={setLimit} />
        <CountField label="每义项例句数" value={perSense} onChange={setPerSense} />
      </div>
      <div className="flex items-center justify-end gap-3">
        <div className="mr-auto">
          <ErrorText error={rows.error} />
        </div>
        <CopyButton
          text={prompt}
          label="复制 prompt"
          onCopied={nooChan.notifyRequest}
          onFailed={nooChan.notifyError}
        />
      </div>
    </div>
  )
}
