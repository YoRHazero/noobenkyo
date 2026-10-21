import type { ImportReport } from '@/bindings'

/** "总计：14；导入：11；跳过：3（片假名词：2；非法：1）" — zero parts left out. */
export function reportSummaryText(report: ImportReport): string {
  const invalid = report.problems.filter((problem) => problem.dropped.kind === 'entry').length
  const kinds: [string, number][] = [
    ['片假名词', report.skipped_katakana],
    ['非法', invalid],
  ]
  const skipped = kinds.reduce((sum, [, n]) => sum + n, 0)
  const ofSkipped = kinds.filter(([, n]) => n > 0).map(([label, n]) => `${label}：${n}`)
  return `总计：${report.lines}；导入：${report.lines - skipped}；跳过：${skipped}${
    ofSkipped.length > 0 ? `（${ofSkipped.join('；')}）` : ''
  }`
}
