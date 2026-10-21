import type { Problem } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { droppedLabel } from '@/features/dictionary/import/report/dropped-label'

type Props = {
  problems: Problem[]
}

/** What the import skipped or left out, by line. */
export function ImportProblemList({ problems }: Props) {
  return (
    <div className="flex flex-col gap-2">
      <table className="w-full table-fixed rounded-md border">
        <thead className="bg-muted text-left text-muted-foreground">
          <tr>
            <th className="w-14 px-3 py-2 font-medium">行</th>
            <th className="w-36 px-3 py-2 font-medium">写法</th>
            <th className="w-36 px-3 py-2 font-medium">类别</th>
            <th className="px-3 py-2 font-medium">原因</th>
          </tr>
        </thead>
        <tbody>
          {problems.map((problem, index) => (
            <tr key={index} className="border-t align-top">
              <td className="px-3 py-2 text-muted-foreground tabular-nums">{problem.line}</td>
              <td className="truncate px-3 py-2" lang="ja">
                {problem.writing}
              </td>
              <td
                className={cn(
                  'px-3 py-2',
                  problem.dropped.kind === 'entry' ? 'text-destructive' : 'text-warning',
                )}
              >
                {droppedLabel(problem.dropped)}
              </td>
              <td className="px-3 py-2">
                {problem.dropped.kind === 'sentence' && (
                  <span lang="ja" className="block font-mono">
                    {problem.dropped.marked}
                  </span>
                )}
                <span className="text-muted-foreground">{problem.reason}</span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
