import { useDictionaryRenderSentence } from '@/backend/dictionary/use-dictionary-render-sentence'
import { SentenceView } from '@/components/sentence-view'

type Props = {
  marked: string
  translation: string | null
}

/** A sentence as it will be shown in full, rendered when its row opens. */
export function PreviewSentence({ marked, translation }: Props) {
  const rendered = useDictionaryRenderSentence(marked, 'Full')
  return (
    <div className="flex flex-col">
      {rendered.data ? (
        <SentenceView fragments={rendered.data} className="text-base" />
      ) : (
        <span lang="ja" className="font-mono text-muted-foreground">
          {marked}
        </span>
      )}
      {rendered.error && <span className="text-destructive">{rendered.error.message}</span>}
      {translation && <span className="text-muted-foreground">{translation}</span>}
    </div>
  )
}
