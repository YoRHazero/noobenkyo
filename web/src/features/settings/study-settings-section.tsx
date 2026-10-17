import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { StudySettingsEditor } from '@/features/settings/study-settings-editor'
import { useResetStudySettings } from '@/features/settings/use-reset-study-settings'
import { useSaveStudySettings } from '@/features/settings/use-save-study-settings'
import { useStudySettings } from '@/features/settings/use-study-settings'

/**
 * Study settings. The editor restarts from the saved settings whenever they
 * change; saving and resetting live here so their outcome outlasts the editor.
 */
export function StudySettingsSection() {
  const settings = useStudySettings()
  const save = useSaveStudySettings()
  const reset = useResetStudySettings()
  return (
    <Section title="学习" description="题型顺序、区的容量和复习权重。">
      {settings.data && (
        <StudySettingsEditor
          key={settings.dataUpdatedAt}
          initial={settings.data}
          pending={save.isPending || reset.isPending}
          onSave={(draft) => {
            reset.reset()
            save.mutate(draft)
          }}
          onReset={() => {
            save.reset()
            reset.mutate()
          }}
        />
      )}
      {(save.isSuccess || reset.isSuccess) && (
        <p className="text-sm text-muted-foreground">已保存</p>
      )}
      <ErrorText error={settings.error ?? save.error ?? reset.error} />
    </Section>
  )
}
