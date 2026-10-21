import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import type { ZoneSetting } from '@/features/settings/settings-draft'

const names: Record<ZoneSetting, string> = {
  capacity: '学习区容量',
  kanjiOrder: '题型顺序（有汉字）',
  kanaOrder: '题型顺序（纯假名）',
}

type Props = {
  /** The changes that clear the zone; the dialog is open while there are some. */
  changes: ZoneSetting[]
  open: boolean
  onOpenChange: (open: boolean) => void
  onSave: () => void
}

/** Asks before a save that clears the learning zone, naming what causes it. */
export function ClearZoneDialog({ changes, open, onOpenChange, onSave }: Props) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent aria-describedby={undefined} className="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>保存设置</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-1 text-sm">
          <p className="text-destructive">学习区会清空</p>
          <p>原因：{changes.map((change) => names[change]).join('、')}</p>
          <p className="text-muted-foreground">词库里的进度不变</p>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">取消</Button>
          </DialogClose>
          <Button
            onClick={() => {
              onOpenChange(false)
              onSave()
            }}
          >
            保存
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
