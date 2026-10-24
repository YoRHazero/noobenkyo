import { DetailItem, DetailList } from '@/components/detail-list'

type Props = {
  /** The line as written. */
  source: string
  reason: string
}

/** Why a line is not an entry, and the line itself. */
export function InvalidRowDetail({ source, reason }: Props) {
  return (
    <DetailList>
      <DetailItem label="原因">{reason}</DetailItem>
      <DetailItem label="原文">
        <span lang="ja" className="font-mono break-all">
          {source}
        </span>
      </DetailItem>
    </DetailList>
  )
}
