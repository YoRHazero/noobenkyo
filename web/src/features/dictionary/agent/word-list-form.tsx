import { useId, useState } from 'react'
import { CopyButton } from '@/components/copy-button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { petFailed, petRequested } from '@/stores/pet'
import { CountField } from '@/features/dictionary/agent/count-field'
import { parsePositiveInteger } from '@/features/dictionary/agent/positive-integer'
import { wordListPrompt } from '@/features/dictionary/agent/word-list-prompt'

/** A new word list on a topic, written down to its sentences; no topic leaves it to the agent. */
export function WordListForm() {
  const topicId = useId()
  const [topic, setTopic] = useState('')
  const [count, setCount] = useState('20')
  const [perSense, setPerSense] = useState('3')
  const countValue = parsePositiveInteger(count)
  const perSenseValue = parsePositiveInteger(perSense)
  const prompt =
    countValue !== null && perSenseValue !== null
      ? wordListPrompt({ topic, count: countValue, perSense: perSenseValue })
      : undefined

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
        <CopyButton
          text={prompt}
          label="复制给 agent"
          onCopied={petRequested}
          onFailed={petFailed}
        />
      </div>
    </div>
  )
}
