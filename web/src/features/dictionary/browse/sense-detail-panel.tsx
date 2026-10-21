import { useState } from 'react'
import type { SenseKey } from '@/bindings'
import { useDictionarySenseDetail } from '@/backend/dictionary/use-dictionary-sense-detail'
import { ErrorText } from '@/components/error-text'
import { SenseDetailView } from '@/features/dictionary/browse/sense-detail-view'
import { SenseEditor } from '@/features/dictionary/browse/sense-editor'

type Props = {
  senseKey: SenseKey
  maxStage: number
  /** The sense's new key, after an edit changed its primary gloss. */
  onRekey: (to: SenseKey) => void
}

/** An open sense: its detail, or the form editing it. */
export function SenseDetailPanel({ senseKey, maxStage, onRekey }: Props) {
  const detail = useDictionarySenseDetail(senseKey)
  const [editing, setEditing] = useState(false)

  if (detail.error) {
    return <ErrorText error={detail.error} />
  }
  if (detail.data === undefined) {
    return <p className="text-sm text-muted-foreground">加载中…</p>
  }
  if (detail.data === null) {
    return <p className="text-sm text-muted-foreground">无</p>
  }
  return editing ? (
    <SenseEditor
      detail={detail.data.detail}
      onCancel={() => setEditing(false)}
      onSaved={(key) => {
        setEditing(false)
        onRekey(key)
      }}
    />
  ) : (
    <SenseDetailView
      detail={detail.data.detail}
      sentences={detail.data.sentences}
      maxStage={maxStage}
      onEdit={() => setEditing(true)}
    />
  )
}
