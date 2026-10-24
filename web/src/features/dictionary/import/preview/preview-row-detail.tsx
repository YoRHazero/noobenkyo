import type { ConvertedEntry } from '@/bindings'
import { Badge } from '@/components/ui/badge'
import { DetailItem, DetailList } from '@/components/detail-list'
import type {
  DroppedLanguage,
  DroppedSentence,
} from '@/features/dictionary/import/preview/preview-rows'
import { PreviewSentence } from '@/features/dictionary/import/preview/preview-sentence'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { progressText } from '@/features/dictionary/progress-text'

type Props = {
  entry: ConvertedEntry
  droppedLanguage: DroppedLanguage | null
  droppedSentences: DroppedSentence[]
  unknownFields: string[]
}

/** Everything an entry brings in, and what of its line is left out. */
export function PreviewRowDetail({
  entry,
  droppedLanguage,
  droppedSentences,
  unknownFields,
}: Props) {
  const { content } = entry
  const sense = content.reading?.sense ?? null
  const maxStage = useStudySettings().data?.quiz.max_stage
  return (
    <DetailList>
      {content.origin_language && <DetailItem label="语种">{content.origin_language}</DetailItem>}
      {droppedLanguage && (
        <DetailItem label="语种">
          <span className="flex flex-col">
            <span className="text-warning">✕ {droppedLanguage.value}</span>
            <span className="text-muted-foreground">{droppedLanguage.reason}</span>
          </span>
        </DetailItem>
      )}
      {sense && (
        <DetailItem label="释义">
          <span className="font-medium">{sense.glosses[0]}</span>
          {sense.glosses.length > 1 && (
            <span className="text-muted-foreground">；{sense.glosses.slice(1).join('；')}</span>
          )}
        </DetailItem>
      )}
      {sense && sense.labels.length > 0 && (
        <DetailItem label="labels">
          <span className="flex flex-wrap gap-1">
            {sense.labels.map((label) => (
              <Badge key={label} variant="secondary">
                {label}
              </Badge>
            ))}
          </span>
        </DetailItem>
      )}
      {sense?.progress && maxStage !== undefined && (
        <DetailItem label="进度">{progressText(sense.progress, maxStage)}</DetailItem>
      )}
      {sense && (sense.sentences.length > 0 || droppedSentences.length > 0) && (
        <DetailItem label="例句">
          <ul className="flex flex-col gap-2">
            {sense.sentences.map((sentence) => (
              <li key={sentence.marked}>
                <PreviewSentence marked={sentence.marked} translation={sentence.translation} />
              </li>
            ))}
            {droppedSentences.map((sentence) => (
              <li key={sentence.marked} className="flex flex-col">
                <span lang="ja" className="font-mono text-warning">
                  ✕ {sentence.marked}
                </span>
                <span className="text-muted-foreground">{sentence.reason}</span>
              </li>
            ))}
          </ul>
        </DetailItem>
      )}
      {unknownFields.length > 0 && (
        <DetailItem label="未知字段">
          <span className="font-mono">{unknownFields.join('、')}</span>
        </DetailItem>
      )}
    </DetailList>
  )
}
