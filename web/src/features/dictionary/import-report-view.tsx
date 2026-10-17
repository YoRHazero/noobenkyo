import type { ImportReport } from '@/bindings'

type Props = {
  report: ImportReport
}

/** Counts of an import and the entries left for the user to handle. */
export function ImportReportView({ report }: Props) {
  const counts = [
    { label: '条目', value: report.entries },
    { label: '新单词', value: report.words_added },
    { label: '新读音', value: report.readings_added },
    { label: '新义项', value: report.senses_added },
    { label: '合并义项', value: report.senses_merged },
    { label: '新例句', value: report.sentences_added },
    { label: '更新进度', value: report.progress_updated },
    { label: '跳过片假名词', value: report.skipped_katakana },
  ]
  return (
    <div className="flex flex-col gap-3">
      <dl className="grid grid-cols-4 gap-3 text-sm">
        {counts.map((count) => (
          <div
            key={count.label}
            className="flex justify-between gap-2 rounded-md bg-muted px-3 py-2"
          >
            <dt className="text-muted-foreground">{count.label}</dt>
            <dd className="tabular-nums">{count.value}</dd>
          </div>
        ))}
      </dl>
      {report.warnings.length > 0 && (
        <div className="flex flex-col gap-1">
          <p className="text-sm font-medium">
            警告（{report.warnings.length}）：这些条目没有导入，请处理后重新导入
          </p>
          <ul className="max-h-64 overflow-auto rounded-md border text-sm">
            {report.warnings.map((warning) => (
              <li key={warning.line} className="flex gap-3 border-b px-3 py-1.5 last:border-b-0">
                <span className="w-16 shrink-0 text-muted-foreground tabular-nums">
                  第 {warning.line} 行
                </span>
                <span lang="ja" className="w-28 shrink-0">
                  {warning.writing}
                </span>
                <span>{warning.message}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  )
}
