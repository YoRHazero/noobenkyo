import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { Button } from '@/components/ui/button'
import { useLogCount } from '@/features/dictionary/use-log-count'
import { useClearLog } from '@/features/dictionary/use-clear-log'
import { useExportLog } from '@/features/dictionary/use-export-log'
import { useImportLog } from '@/features/dictionary/use-import-log'

/** The answer log: export, import (union) and clearing. The switch lives in settings. */
export function AnswerLogPanel() {
  const count = useLogCount()
  const exportLog = useExportLog()
  const importLog = useImportLog()
  const clearLog = useClearLog()
  const pending = exportLog.isPending || importLog.isPending || clearLog.isPending

  return (
    <Section title="答题记录" description="只记错题和改判。是否记录在设置里开关。">
      <p className="text-sm">已有 {count.data ?? 0} 条记录</p>
      <div className="flex gap-3">
        <Button variant="outline" disabled={pending} onClick={() => exportLog.mutate(undefined)}>
          导出…
        </Button>
        <Button variant="outline" disabled={pending} onClick={() => importLog.mutate(undefined)}>
          导入…
        </Button>
        <Button variant="outline" disabled={pending} onClick={() => clearLog.mutate(undefined)}>
          清空
        </Button>
      </div>
      {typeof exportLog.data === 'number' && (
        <p className="text-sm text-muted-foreground">已导出 {exportLog.data} 条</p>
      )}
      {importLog.data && (
        <p className="text-sm text-muted-foreground">
          读入 {importLog.data.lines} 行，新增 {importLog.data.added} 条，无效{' '}
          {importLog.data.invalid} 行
        </p>
      )}
      <ErrorText error={count.error ?? exportLog.error ?? importLog.error ?? clearLog.error} />
    </Section>
  )
}
