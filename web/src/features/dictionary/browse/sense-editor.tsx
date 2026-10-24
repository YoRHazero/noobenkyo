import { useState } from 'react'
import type { SenseDetail, SenseKey, SensePatch } from '@/bindings'
import { Plus } from 'lucide-react'
import { useDictionaryReplaceSense } from '@/backend/dictionary/use-dictionary-replace-sense'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { SentenceField } from '@/features/dictionary/browse/sentence-field'
import { joinList, splitList } from '@/features/dictionary/browse/text-list'
import { DetailItem, DetailList } from '@/components/detail-list'

type SentenceDraft = {
  /** Keeps a field's identity while the ones above it are removed. */
  id: string
  marked: string
  translation: string
}

type Props = {
  detail: SenseDetail
  onCancel: () => void
  /** The sense's key after saving, new when the primary gloss changed. */
  onSaved: (key: SenseKey) => void
}

/**
 * Edits the glosses, labels and sentences of a sense, saved together as one
 * replacement. A rejected save changes nothing and keeps what was typed.
 */
export function SenseEditor({ detail, onCancel, onSaved }: Props) {
  const [glosses, setGlosses] = useState(joinList(detail.glosses))
  const [labels, setLabels] = useState(joinList(detail.labels))
  const [sentences, setSentences] = useState<SentenceDraft[]>(() =>
    detail.sentences.map((sentence) => ({
      id: crypto.randomUUID(),
      marked: sentence.marked,
      translation: sentence.translation ?? '',
    })),
  )
  const replace = useDictionaryReplaceSense()
  const replaced = replace.data

  const save = async () => {
    const patch: SensePatch = {
      glosses: splitList(glosses),
      labels: splitList(labels),
      sentences: sentences
        // A field added and left empty is no sentence.
        .filter((sentence) => sentence.marked.trim() !== '' || sentence.translation.trim() !== '')
        .map((sentence) => ({
          marked: sentence.marked.trim(),
          translation: sentence.translation.trim() || null,
        })),
    }
    // The editor may be gone by then: a new primary gloss is a new row.
    const result = await replace.mutateAsync({ key: detail.key, patch }).catch(() => null)
    if (result?.kind === 'done') {
      onSaved(result.key)
    }
  }
  const change = (id: string, marked: string, translation: string) =>
    setSentences(sentences.map((s) => (s.id === id ? { ...s, marked, translation } : s)))

  return (
    <div className="flex flex-col gap-3">
      <DetailList>
        <DetailItem label="释义">
          <div className="flex flex-col gap-1">
            <Input
              value={glosses}
              aria-label="释义"
              className="h-8"
              onChange={(event) => setGlosses(event.target.value)}
            />
            <span className="text-xs text-muted-foreground">用"；"分开，第一个是主释义</span>
          </div>
        </DetailItem>
        <DetailItem label="labels">
          <Input
            value={labels}
            aria-label="labels"
            className="h-8"
            onChange={(event) => setLabels(event.target.value)}
          />
        </DetailItem>
        <DetailItem label="例句">
          <div className="flex flex-col gap-3">
            {sentences.map((sentence) => (
              <SentenceField
                key={sentence.id}
                marked={sentence.marked}
                translation={sentence.translation}
                onChange={(marked, translation) => change(sentence.id, marked, translation)}
                onRemove={() => setSentences(sentences.filter((s) => s.id !== sentence.id))}
              />
            ))}
            <Button
              variant="outline"
              size="sm"
              className="self-start"
              onClick={() =>
                setSentences([
                  ...sentences,
                  { id: crypto.randomUUID(), marked: '', translation: '' },
                ])
              }
            >
              <Plus />
              添加例句
            </Button>
          </div>
        </DetailItem>
      </DetailList>
      {replaced?.kind === 'rejected' && (
        <ul className="flex flex-col gap-1 text-sm text-destructive">
          {replaced.problems.map((problem) => (
            <li key={problem}>{problem}</li>
          ))}
        </ul>
      )}
      {replaced?.kind === 'notFound' && <p className="text-sm text-destructive">义项已不存在</p>}
      <ErrorText error={replace.error} />
      <div className="flex gap-2">
        <Button variant="outline" size="sm" disabled={replace.isPending} onClick={onCancel}>
          取消
        </Button>
        <Button size="sm" disabled={replace.isPending} onClick={() => void save()}>
          保存
        </Button>
      </div>
    </div>
  )
}
