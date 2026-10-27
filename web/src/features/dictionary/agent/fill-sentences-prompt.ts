import type { ProgressState } from '@/bindings'
import { shellWord } from '@/features/dictionary/agent/shell-word'

/**
 * Sentences to write for senses that have none, from the rows the
 * dictionary page's keyword and progress filter show.
 */
export type FillSentencesTask = {
  /** Blank or `null` for no keyword. */
  keyword: string | null
  progress: ProgressState
  /** How many rows to take, in the page's order. */
  limit: number
  /** How many sentences each sense gets. */
  perSense: number
}

/** The export's progress option; none when progress does not narrow. */
const progressOption: Record<ProgressState, string | null> = {
  Any: null,
  Untouched: '--progress untouched',
  Touched: '--progress touched',
}

/**
 * The starting prompt for filling in sentences and sending the rows to the
 * app. The rows are given as the export command that yields them, so they
 * are exactly the ones the user chose.
 */
export function fillSentencesPrompt(task: FillSentencesTask): string {
  const exportCommand = ['noobenkyo dictionary export --completeness no-sentence']
  const keyword = task.keyword?.trim() ?? ''
  if (keyword !== '') {
    exportCommand.push(`--keyword ${shellWord(keyword)}`)
  }
  const progress = progressOption[task.progress]
  if (progress !== null) {
    exportCommand.push(progress)
  }
  exportCommand.push(`--limit ${task.limit}`)
  return `用 noobenkyo 命令行给词库里还没有例句的义项补例句，送进 app。用法先运行 \`noobenkyo dictionary guide\` 看。\n\n要补的行：\`${exportCommand.join(' ')}\`\n每个义项补 ${task.perSense} 条例句，其余字段不改。`
}
