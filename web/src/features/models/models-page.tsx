import { useAiDefaultSettings } from '@/backend/ai/use-ai-default-settings'
import { useAiMachineReports } from '@/backend/ai/use-ai-machine-reports'
import { useAiMachinesProbed } from '@/backend/ai/use-ai-machines-probed'
import { useAiProbeMachines } from '@/backend/ai/use-ai-probe-machines'
import { useAiSettings } from '@/backend/ai/use-ai-settings'
import { ErrorText } from '@/components/error-text'
import { Page } from '@/components/page'
import { Button } from '@/components/ui/button'
import { ModelsForm } from '@/features/models/models-form'

/**
 * The machines the app's models run on, and the model each use runs on.
 * The form starts over from what is saved each time the settings are read
 * again, which is how a save shows as done. 刷新 probes the saved machines
 * again; the app probed them as it started, and a save does too, since the
 * directories may have changed.
 */
export function ModelsPage() {
  useAiMachinesProbed()
  const settings = useAiSettings()
  const defaults = useAiDefaultSettings()
  const reports = useAiMachineReports()
  const probeAll = useAiProbeMachines()
  return (
    <Page
      title="模型"
      actions={
        <Button
          variant="outline"
          size="sm"
          disabled={probeAll.isPending}
          onClick={() => probeAll.mutate()}
        >
          刷新
        </Button>
      }
    >
      <ErrorText error={settings.error ?? defaults.error ?? reports.error ?? probeAll.error} />
      {settings.data && defaults.data ? (
        <ModelsForm
          key={settings.dataUpdatedAt}
          saved={settings.data}
          defaults={defaults.data}
          reports={reports.data ?? []}
          reportsAt={reports.dataUpdatedAt}
          probingAll={probeAll.isPending}
          onSaved={() => probeAll.mutate()}
        />
      ) : (
        !settings.error && <p className="text-sm text-muted-foreground">加载中…</p>
      )}
    </Page>
  )
}
