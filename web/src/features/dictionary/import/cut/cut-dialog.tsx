import { useMemo, useState } from 'react'
import type { ImportOptions } from '@/bindings'
import { useDictionaryCheckEntries } from '@/backend/dictionary/use-dictionary-check-entries'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { cutLines } from '@/features/dictionary/import/cut/cut-lines'
import { fixPrompt } from '@/features/dictionary/import/cut/fix-prompt'
import { writeClipboard } from '@/components/write-clipboard'

/** What is about to be cut. */
export type Cutting = {
  /** "活用形", "警告" … */
  label: string
  lines: number[]
}

type Props = {
  /** `null` while nothing is being cut. */
  cutting: Cutting | null
  text: string
  options: ImportOptions
  onClose: () => void
  /** Receives the text without the cut lines. */
  onCut: (rest: string) => void
}

/**
 * Confirms a cut and chooses what goes on the clipboard: the lines as
 * written, or a prompt for a chat AI to fix and complete them.
 */
export function CutDialog({ cutting, onClose, ...rest }: Props) {
  return (
    <Dialog open={cutting !== null} onOpenChange={(open) => !open && onClose()}>
      {cutting && <CutDialogContent cutting={cutting} {...rest} />}
    </Dialog>
  )
}

type ContentProps = Omit<Props, 'cutting' | 'onClose'> & { cutting: Cutting }

/** The lines leave the text only once the clipboard has them. */
function CutDialogContent({ cutting, text, options, onCut }: ContentProps) {
  const { cut, rest } = useMemo(() => cutLines(text, cutting.lines), [text, cutting.lines])
  // Checked on their own, so the warnings number the cut lines from 1.
  const warnings = useDictionaryCheckEntries(cut.join('\n'), options)
  const [error, setError] = useState<Error | null>(null)

  const put = (clipboard: string) => {
    writeClipboard(clipboard).then(() => onCut(rest), setError)
  }

  return (
    <DialogContent aria-describedby={undefined}>
      <DialogHeader>
        <DialogTitle>
          剪切：{cutting.label} {cut.length} 行
        </DialogTitle>
      </DialogHeader>
      <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm">
        <dt className="font-medium">剪切原文</dt>
        <dd className="text-muted-foreground">任意位置粘贴以恢复</dd>
        <dt className="font-medium">生成提示词</dt>
        <dd className="text-muted-foreground">可返回修复结果</dd>
      </dl>
      <ErrorText error={error ?? warnings.error} />
      <DialogFooter>
        <DialogClose asChild>
          <Button variant="outline">取消</Button>
        </DialogClose>
        <Button variant="outline" onClick={() => put(`${cut.join('\n')}\n`)}>
          剪切原文
        </Button>
        <Button
          disabled={!warnings.data}
          onClick={() => warnings.data && put(fixPrompt(warnings.data, cut))}
        >
          生成提示词
        </Button>
      </DialogFooter>
    </DialogContent>
  )
}
