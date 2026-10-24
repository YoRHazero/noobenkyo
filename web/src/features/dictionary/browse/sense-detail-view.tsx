import type { Fragment, SenseDetail } from '@/bindings'
import { SentenceView } from '@/components/sentence-view'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { DeleteSenseButtons } from '@/features/dictionary/browse/delete-sense-buttons'
import { ProgressActions } from '@/features/dictionary/browse/progress-actions'
import { DetailItem, DetailList } from '@/components/detail-list'
import { progressText } from '@/features/dictionary/progress-text'

type Props = {
  detail: SenseDetail
  /** Rendered in full, in the order of `detail.sentences`. */
  sentences: Fragment[][]
  maxStage: number
  onEdit: () => void
}

/** Everything about a sense, with its progress actions, edit and deletes. */
export function SenseDetailView({ detail, sentences, maxStage, onEdit }: Props) {
  const [primary, ...secondary] = detail.glosses
  return (
    <div className="flex flex-col gap-3">
      <DetailList>
        <DetailItem label="释义">
          <span className="font-medium">{primary}</span>
          {secondary.length > 0 && (
            <span className="text-muted-foreground">；{secondary.join('；')}</span>
          )}
        </DetailItem>
        {detail.labels.length > 0 && (
          <DetailItem label="labels">
            <span className="flex flex-wrap gap-1">
              {detail.labels.map((label) => (
                <Badge key={label} variant="secondary">
                  {label}
                </Badge>
              ))}
            </span>
          </DetailItem>
        )}
        <DetailItem label="进度">
          <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
            <span>{progressText(detail.progress, maxStage)}</span>
            <ProgressActions senseKey={detail.key} progress={detail.progress} maxStage={maxStage} />
          </div>
        </DetailItem>
        <DetailItem label="例句">
          {sentences.length === 0 ? (
            <span className="text-muted-foreground">无</span>
          ) : (
            <ul className="flex flex-col gap-2">
              {detail.sentences.map((sentence, index) => {
                const fragments = sentences[index]
                return (
                  <li key={sentence.marked} className="flex flex-col">
                    {fragments && <SentenceView fragments={fragments} className="text-base" />}
                    {sentence.translation && (
                      <span className="text-muted-foreground">{sentence.translation}</span>
                    )}
                  </li>
                )
              })}
            </ul>
          )}
        </DetailItem>
      </DetailList>
      <div className="flex flex-wrap items-center gap-1">
        <Button variant="outline" size="sm" onClick={onEdit}>
          编辑
        </Button>
        <DeleteSenseButtons detail={detail} />
      </div>
    </div>
  )
}
