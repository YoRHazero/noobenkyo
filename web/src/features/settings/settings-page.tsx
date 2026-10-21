import { useStudyDefaultSettings } from '@/backend/study/use-study-default-settings'
import { useStudySettings } from '@/backend/study/use-study-settings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { SettingsForm } from '@/features/settings/settings-form'

/**
 * The study settings. The form starts over from what is saved each time the
 * settings are read again, which is how a save shows as done.
 */
export function SettingsPage() {
  const settings = useStudySettings()
  const defaults = useStudyDefaultSettings()
  return (
    <Page title="设置">
      <ErrorText error={settings.error ?? defaults.error} />
      {settings.data && defaults.data ? (
        <SettingsForm key={settings.dataUpdatedAt} saved={settings.data} defaults={defaults.data} />
      ) : (
        !settings.error && <p className="text-sm text-muted-foreground">加载中…</p>
      )}
    </Page>
  )
}
