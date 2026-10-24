import { useState } from 'react'
import { Pencil } from 'lucide-react'
import { useListeningUpdateSetting } from '@/backend/listening/use-listening-update-setting'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'

type Props = {
  world: string
  setting: string
}

/** The setting in small print under the world's name, edited in place. */
export function WorldSetting({ world, setting }: Props) {
  const [draft, setDraft] = useState<string | null>(null)
  const update = useListeningUpdateSetting()

  if (draft === null) {
    return (
      <div className="flex items-start gap-1">
        <p className="text-sm whitespace-pre-wrap text-muted-foreground">{setting || '无'}</p>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="编辑设定"
          className="-mt-1.5 shrink-0"
          onClick={() => setDraft(setting)}
        >
          <Pencil />
        </Button>
      </div>
    )
  }
  return (
    <div className="flex flex-col gap-2">
      <Textarea
        aria-label="设定"
        value={draft}
        disabled={update.isPending}
        className="min-h-28"
        onChange={(event) => setDraft(event.target.value)}
      />
      <div className="flex items-center justify-end gap-2">
        <ErrorText error={update.error} />
        <Button
          variant="outline"
          size="sm"
          disabled={update.isPending}
          onClick={() => setDraft(null)}
        >
          取消
        </Button>
        <Button
          size="sm"
          disabled={update.isPending}
          onClick={() =>
            update.mutate({ name: world, setting: draft }, { onSuccess: () => setDraft(null) })
          }
        >
          保存
        </Button>
      </div>
    </div>
  )
}
