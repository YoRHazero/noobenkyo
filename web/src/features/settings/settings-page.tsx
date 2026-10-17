import { Page } from '@/components/page'
import { DictionarySettingsSection } from '@/features/settings/dictionary-settings-section'
import { StudySettingsSection } from '@/features/settings/study-settings-section'

export function SettingsPage() {
  return (
    <Page title="设置">
      <StudySettingsSection />
      <DictionarySettingsSection />
    </Page>
  )
}
