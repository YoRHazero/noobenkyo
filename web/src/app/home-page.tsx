import { Page } from '@/components/page'
import { DictionaryOverview } from '@/features/dictionary/dictionary-overview'
import { ZoneEntrances } from '@/features/study/zone-entrances'

export function HomePage() {
  return (
    <Page title="今天">
      <ZoneEntrances />
      <DictionaryOverview />
    </Page>
  )
}
