import { useState } from 'react'
import type { ImportOptions } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { Button } from '@/components/ui/button'
import { ImportReportView } from '@/features/dictionary/import-report-view'
import { LabeledCheckbox } from '@/components/labeled-checkbox'
import { useImportEntries } from '@/features/dictionary/use-import-entries'

const defaultOptions: ImportOptions = {
  drop_pure_katakana: true,
  detect_inflections: false,
  keep_progress: true,
}

/** Importing a JSON Lines file of entries. */
export function ImportPanel() {
  const [options, setOptions] = useState(defaultOptions)
  const importEntries = useImportEntries()
  const set = (key: keyof ImportOptions) => (value: boolean) =>
    setOptions({ ...options, [key]: value })

  return (
    <Section title="导入" description="JSON Lines 文件，一行一个条目。">
      <div className="flex flex-wrap gap-6">
        <LabeledCheckbox
          label="去掉纯片假名词"
          checked={options.drop_pure_katakana}
          onChange={set('drop_pure_katakana')}
        />
        <LabeledCheckbox
          label="识别活用形"
          checked={options.detect_inflections}
          onChange={set('detect_inflections')}
        />
        <LabeledCheckbox
          label="保留进度"
          checked={options.keep_progress}
          onChange={set('keep_progress')}
        />
      </div>
      <div className="flex items-center gap-4">
        <Button disabled={importEntries.isPending} onClick={() => importEntries.mutate(options)}>
          选择文件并导入
        </Button>
        {importEntries.isPending && <span className="text-sm text-muted-foreground">导入中…</span>}
      </div>
      <ErrorText error={importEntries.error} />
      {importEntries.data && <ImportReportView report={importEntries.data} />}
    </Section>
  )
}
