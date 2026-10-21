import type { ImportReport } from '@/bindings'
import { Button } from '@/components/ui/button'
import { DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { ImportProblemList } from '@/features/dictionary/import/report/import-problem-list'
import { reportSummaryText } from '@/features/dictionary/import/report/report-summary-text'

type Props = {
  report: ImportReport
  /** Back to the text and options as they were, to fix and import again. */
  onBack: () => void
  onClose: () => void
}

/** What the import did: the text first, then what changed in the dictionary. */
export function ImportReportView({ report, onBack, onClose }: Props) {
  return (
    <>
      <DialogHeader>
        <DialogTitle>导入完成</DialogTitle>
      </DialogHeader>
      <div className="flex min-h-0 flex-1 flex-col gap-4 overflow-auto text-sm">
        <p>{reportSummaryText(report)}</p>
        <div className="flex flex-col gap-2">
          <Counts
            title="新建"
            counts={[
              ['单词', report.words_added],
              ['读音', report.readings_added],
              ['义项', report.senses_added],
              ['例句', report.sentences_added],
            ]}
          />
          <Counts
            title="并入"
            counts={[
              ['义项', report.senses_merged],
              ['进度', report.progress_updated],
            ]}
          />
        </div>
        {report.problems.length > 0 && <ImportProblemList problems={report.problems} />}
      </div>
      <DialogFooter>
        <Button variant="outline" onClick={onBack}>
          回到文本
        </Button>
        <Button onClick={onClose}>关闭</Button>
      </DialogFooter>
    </>
  )
}

function Counts({ title, counts }: { title: string; counts: [string, number][] }) {
  return (
    <div className="flex items-baseline gap-6">
      <span className="w-10 text-muted-foreground">{title}</span>
      {counts.map(([label, value]) => (
        <span key={label}>
          {label} <span className="text-lg font-semibold tabular-nums">{value}</span>
        </span>
      ))}
    </div>
  )
}
