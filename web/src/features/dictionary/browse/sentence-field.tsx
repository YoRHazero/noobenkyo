import { X } from 'lucide-react'
import { useDictionaryRenderSentence } from '@/backend/dictionary/use-dictionary-render-sentence'
import { SentenceView } from '@/components/sentence-view'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

type Props = {
  marked: string
  translation: string
  onChange: (marked: string, translation: string) => void
  onRemove: () => void
}

/** One sentence being edited: its marked text, its translation, and how it renders. */
export function SentenceField({ marked, translation, onChange, onRemove }: Props) {
  return (
    <div className="flex items-start gap-2">
      <div className="flex min-w-0 flex-1 flex-col gap-1.5">
        <Input
          lang="ja"
          value={marked}
          placeholder="标记文本"
          aria-label="标记文本"
          className="h-8 font-mono"
          onChange={(event) => onChange(event.target.value, translation)}
        />
        <Input
          value={translation}
          placeholder="翻译"
          aria-label="翻译"
          className="h-8"
          onChange={(event) => onChange(marked, event.target.value)}
        />
        {marked.trim() !== '' && <MarkedPreview marked={marked.trim()} />}
      </div>
      <Button variant="ghost" size="icon-sm" aria-label="删除这条例句" onClick={onRemove}>
        <X />
      </Button>
    </div>
  )
}

/** The sentence as it will show, or why its marks are wrong. */
function MarkedPreview({ marked }: { marked: string }) {
  const rendered = useDictionaryRenderSentence(marked, 'Full')
  if (rendered.error) {
    return <span className="text-sm text-destructive">{rendered.error.message}</span>
  }
  return rendered.data ? <SentenceView fragments={rendered.data} className="text-base" /> : null
}
