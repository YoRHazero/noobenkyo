import { useState } from 'react'
import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { Button } from '@/components/ui/button'
import { LabeledCheckbox } from '@/components/labeled-checkbox'
import { useExportEntries } from '@/features/dictionary/use-export-entries'

/** Exporting the dictionary as entries, for another computer. */
export function ExportPanel() {
  const [keepProgress, setKeepProgress] = useState(true)
  const exportEntries = useExportEntries()
  return (
    <Section title="导出" description="每个义项一行。在另一台电脑上导入即可。">
      <LabeledCheckbox label="保留进度" checked={keepProgress} onChange={setKeepProgress} />
      <div className="flex items-center gap-4">
        <Button
          disabled={exportEntries.isPending}
          onClick={() => exportEntries.mutate(keepProgress)}
        >
          导出…
        </Button>
        {exportEntries.data && (
          <span className="text-sm text-muted-foreground">
            已导出 {exportEntries.data.entries} 行
          </span>
        )}
      </div>
      <ErrorText error={exportEntries.error} />
    </Section>
  )
}
