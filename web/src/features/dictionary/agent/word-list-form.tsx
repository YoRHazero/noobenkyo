import { useId, useState } from 'react'
import { useAiWordListPrompt } from '@/backend/ai/use-ai-word-list-prompt'
import { CopyButton } from '@/components/copy-button'
import { ErrorText } from '@/components/error-text'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { CountField } from '@/features/dictionary/agent/count-field'
import { parsePositiveInteger } from '@/features/dictionary/agent/positive-integer'

/** A new word list on a topic, written down to its sentences; no topic leaves it to the agent. */
export function WordListForm() {
  const topicId = useId()
  const [topic, setTopic] = useState('')
  const [count, setCount] = useState('20')
  const [perSense, setPerSense] = useState('3')
  const countValue = parsePositiveInteger(count)
  const perSenseValue = parsePositiveInteger(perSense)
  const prompt = useAiWordListPrompt({
    topic,
    count: countValue ?? 0,
    per_sense: perSenseValue ?? 0,
  })
  const ready = countValue !== null && perSenseValue !== null

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-col gap-2">
        <div className="flex items-center gap-3">
          <Label htmlFor={topicId} className="w-24">
            话题
          </Label>
          <Input
            id={topicId}
            placeholder="空着：交给 agent 判断"
            value={topic}
            className="h-8 flex-1"
            onChange={(event) => setTopic(event.target.value)}
          />
        </div>
        <CountField label="个数" value={count} onChange={setCount} />
        <CountField label="每个义项几条" value={perSense} onChange={setPerSense} />
      </div>
      <div className="flex items-center justify-end gap-3">
        <div className="mr-auto">
          <ErrorText error={prompt.error} />
        </div>
        <CopyButton text={ready ? prompt.data : undefined} label="复制给 agent" />
      </div>
    </div>
  )
}
