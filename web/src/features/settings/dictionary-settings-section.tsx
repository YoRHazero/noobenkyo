import { useId } from 'react'
import { ErrorText } from '@/components/error-text'
import { Section } from '@/components/section'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { useDictionarySettings } from '@/features/settings/use-dictionary-settings'
import { useSaveDictionarySettings } from '@/features/settings/use-save-dictionary-settings'

/** Dictionary settings: the answer log switch, saved at once. */
export function DictionarySettingsSection() {
  const id = useId()
  const settings = useDictionarySettings()
  const save = useSaveDictionarySettings()
  return (
    <Section title="答题记录" description="只记错题和改判。关闭后已有记录保留，可以在词库页清空。">
      {settings.data && (
        <div className="flex items-center gap-3">
          <Switch
            id={id}
            checked={settings.data.log_enabled}
            disabled={save.isPending}
            onCheckedChange={(log_enabled) => save.mutate({ ...settings.data, log_enabled })}
          />
          <Label htmlFor={id}>记录答题</Label>
        </div>
      )}
      <ErrorText error={settings.error ?? save.error} />
    </Section>
  )
}
