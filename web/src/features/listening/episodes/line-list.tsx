import { useState } from 'react'
import type { LineView } from '@/bindings'
import { useListeningDeleteLines } from '@/backend/listening/use-listening-delete-lines'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'
import { LineRow } from '@/features/listening/episodes/line-row'
import { clickLine, noSelection } from '@/features/listening/episodes/line-selection'

type Props = {
  lines: LineView[]
}

/**
 * The episode's lines, in order. Lines are ticked to be deleted together;
 * none can be inserted or moved.
 */
export function LineList({ lines }: Props) {
  const [selection, setSelection] = useState(noSelection)
  const remove = useListeningDeleteLines()
  const order = lines.map((line) => line.id)
  // Lines deleted elsewhere drop out of the selection.
  const ticked = order.filter((id) => selection.ids.has(id))

  return (
    <div className="flex flex-col gap-2">
      {ticked.length > 0 && (
        <div className="sticky top-0 z-10 flex items-center justify-between rounded-lg border bg-card px-4 py-2 text-sm">
          <span>已选：{ticked.length} 行</span>
          <div className="flex items-center gap-3">
            <ErrorText error={remove.error} />
            <ConfirmButton
              label="删除"
              confirmLabel={`确认删除 ${ticked.length} 行`}
              disabled={remove.isPending}
              onConfirm={() =>
                remove.mutate(ticked, { onSuccess: () => setSelection(noSelection) })
              }
            />
          </div>
        </div>
      )}
      <ol className="flex flex-col divide-y">
        {lines.map((line, index) => (
          <LineRow
            key={line.id}
            line={line}
            number={index + 1}
            ticked={selection.ids.has(line.id)}
            onTick={(shift) => setSelection(clickLine(selection, order, line.id, shift))}
          />
        ))}
      </ol>
    </div>
  )
}
