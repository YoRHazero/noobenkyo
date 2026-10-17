import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { StudySettingsEditor } from '@/features/settings/study-settings-editor'
import { useStudySettings } from '@/features/settings/use-study-settings'

/** Study settings; the editor restarts from the saved settings whenever they change. */
export function StudySettingsSection() {
  const settings = useStudySettings()
  return (
    <Section title="学习" description="题型顺序、区的容量和复习权重。">
      <ErrorText error={settings.error} />
      {settings.data && (
        <StudySettingsEditor key={settings.dataUpdatedAt} initial={settings.data} />
      )}
    </Section>
  )
}
