import { useState } from 'react'
import type { ContentFilter } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { LabeledCheckbox } from '@/components/labeled-checkbox'
import { Button } from '@/components/ui/button'
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { useExportToFile } from '@/features/dictionary/export/use-export-to-file'

type Props = {
  filter: ContentFilter
  total: number
}

const isEmpty = (filter: ContentFilter) =>
  !filter.text?.trim() && filter.completeness.length === 0 && filter.progress === 'Any'

/**
 * Exporting the rows the dictionary page shows. Progress is left out unless
 * asked for: an export is usually shared with others.
 */
export function ExportDialog({ filter, total }: Props) {
  const [keepProgress, setKeepProgress] = useState(false)
  const exportToFile = useExportToFile()
  const done = exportToFile.data

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>导出</DialogTitle>
        <DialogDescription>
          {isEmpty(filter) ? `范围：全部（${total} 行）` : `范围：当前筛选（${total} 行）`}
        </DialogDescription>
      </DialogHeader>
      {done ? (
        <div className="flex flex-col gap-1 text-sm">
          <p>已导出：{done.entries} 行</p>
          <p className="break-all text-muted-foreground">位置：{done.path}</p>
        </div>
      ) : (
        <LabeledCheckbox
          label="保留进度"
          checked={keepProgress}
          disabled={exportToFile.isPending}
          onChange={setKeepProgress}
        />
      )}
      <ErrorText error={exportToFile.error} />
      <DialogFooter>
        {done ? (
          <DialogClose asChild>
            <Button>关闭</Button>
          </DialogClose>
        ) : (
          <>
            <DialogClose asChild>
              <Button variant="outline">取消</Button>
            </DialogClose>
            <Button
              disabled={total === 0 || exportToFile.isPending}
              onClick={() => exportToFile.mutate({ filter, keepProgress })}
            >
              导出…
            </Button>
          </>
        )}
      </DialogFooter>
    </DialogContent>
  )
}
