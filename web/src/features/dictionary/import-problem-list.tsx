import type { Problem } from '@/bindings'
import { droppedLabel } from '@/features/dictionary/dropped-label'

type Props = {
  problems: Problem[]
}

/** What an import took out, line by line. */
export function ImportProblemList({ problems }: Props) {
  return (
    <div className="flex flex-col gap-1">
      <p className="text-sm font-medium">问题（{problems.length}）：下面这些内容没有导入</p>
      <ul className="max-h-64 overflow-auto rounded-md border text-sm">
        {problems.map((problem, index) => (
          <li key={index} className="flex gap-3 border-b px-3 py-1.5 last:border-b-0">
            <span className="w-16 shrink-0 text-muted-foreground tabular-nums">
              第 {problem.line} 行
            </span>
            <span lang="ja" className="w-28 shrink-0">
              {problem.writing}
            </span>
            <span className="w-20 shrink-0">{droppedLabel(problem.dropped)}</span>
            <span className="flex flex-col">
              {problem.dropped.kind === 'sentence' && (
                <span lang="ja">{problem.dropped.marked}</span>
              )}
              <span className="text-muted-foreground">{problem.reason}</span>
            </span>
          </li>
        ))}
      </ul>
    </div>
  )
}
