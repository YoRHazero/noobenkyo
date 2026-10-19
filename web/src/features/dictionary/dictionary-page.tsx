import { Page } from '@/components/page'
import { DictionaryOverview } from '@/features/dictionary/dictionary-overview'
import { ExportPanel } from '@/features/dictionary/export-panel'
import { ImportPanel } from '@/features/dictionary/import-panel'
import { SenseBrowser } from '@/features/dictionary/sense-browser'

export function DictionaryPage() {
  return (
    <Page title="词库">
      <DictionaryOverview />
      <ImportPanel />
      <ExportPanel />
      <SenseBrowser />
    </Page>
  )
}
