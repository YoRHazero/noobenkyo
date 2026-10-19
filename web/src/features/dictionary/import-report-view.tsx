import type { ImportReport } from '@/bindings'
import { ImportProblemList } from '@/features/dictionary/import-problem-list'

type Props = {
  report: ImportReport
}

/** Counts of an import, what it took out, and the fields it ignored. */
export function ImportReportView({ report }: Props) {
  const counts = [
    { label: '条目', value: report.lines },
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
      {report.problems.length > 0 && <ImportProblemList problems={report.problems} />}
      {report.ignored_fields.length > 0 && (
        <p className="text-sm text-muted-foreground">
          忽略了不认识的字段：
          {report.ignored_fields.map((field) => `${field.name}（${field.lines} 行）`).join('、')}
        </p>
      )}
    </div>
  )
}
