import type { MachineReport } from '@/bindings'
import { ErrorText } from '@/components/error-text'
import { cn } from '@/components/ui/cn'

type Props = {
  report: MachineReport | null
  checking: boolean
  /** Why 检查 was refused: the machine as it stands could not be saved either. */
  error: Error | null
}

/** Whether the machine answered, and what it lacks where the form says to look. */
export function MachineStatus({ report, checking, error }: Props) {
  if (checking) {
    return <Status tone="muted">检查中…</Status>
  }
  if (error) {
    return <ErrorText error={error} />
  }
  if (report === null) {
    return <Status tone="muted">未检查</Status>
  }
  if (report.kind === 'offline') {
    return <Status tone="bad">离线：{report.reason}</Status>
  }
  const problems = [
    report.models_dir === false && '模型文件夹不存在',
    report.qwen_tts?.problem && `qwen-tts：${report.qwen_tts.problem}`,
    report.llama_cpp?.problem && `llama.cpp：${report.llama_cpp.problem}`,
  ].filter((problem) => typeof problem === 'string')
  return (
    <div className="flex min-w-0 flex-col">
      <Status tone="good">在线</Status>
      {problems.map((problem) => (
        <Status key={problem} tone="bad">
          {problem}
        </Status>
      ))}
    </div>
  )
}

const tones = {
  muted: 'text-muted-foreground',
  good: 'text-emerald-600 dark:text-emerald-400',
  bad: 'text-destructive',
}

function Status({ tone, children }: { tone: keyof typeof tones; children: React.ReactNode }) {
  return (
    <span className={cn('text-sm break-words whitespace-pre-wrap', tones[tone])}>{children}</span>
  )
}
