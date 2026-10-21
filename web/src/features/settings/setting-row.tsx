import type { ReactNode } from 'react'
import { RotateCcw } from 'lucide-react'
import { Button } from '@/components/ui/button'

type Props = {
  label: string
  /** The default as text, shown with its restore button when the value differs. */
  defaultText: string
  isDefault: boolean
  onRestore: () => void
  children: ReactNode
}

/** One setting: its name, its control, and its default when it differs. */
export function SettingRow({ label, defaultText, isDefault, onRestore, children }: Props) {
  return (
    <div className="grid grid-cols-[9rem_1fr] items-center gap-x-4 py-2">
      <span className="text-sm text-muted-foreground">{label}</span>
      <div className="flex min-h-8 flex-wrap items-center gap-x-4 gap-y-2">
        {children}
        {!isDefault && (
          <span className="flex items-center gap-0.5 text-xs text-muted-foreground">
            默认：{defaultText}
            <Button
              variant="ghost"
              size="icon-xs"
              aria-label={`恢复${label}的默认值`}
              onClick={onRestore}
            >
              <RotateCcw />
            </Button>
          </span>
        )}
      </div>
    </div>
  )
}
