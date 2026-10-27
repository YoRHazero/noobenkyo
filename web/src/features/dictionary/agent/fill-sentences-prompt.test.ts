import { describe, expect, it } from 'vitest'
import { fillSentencesPrompt } from '@/features/dictionary/agent/fill-sentences-prompt'

describe('fillSentencesPrompt', () => {
  it('gives the rows as the export of the filter', () => {
    const prompt = fillSentencesPrompt({
      keyword: ' 猫 ',
      progress: 'Any',
      limit: 20,
      perSense: 2,
    })
    expect(prompt)
      .toBe(`用 noobenkyo 命令行给词库里还没有例句的义项补例句，送进 app。用法先运行 \`noobenkyo dictionary guide\` 看。

要补的行：\`noobenkyo dictionary export --completeness no-sentence --keyword 猫 --limit 20\`
每个义项补 2 条例句，其余字段不改。`)
  })

  it('leaves out a blank keyword and keeps progress', () => {
    const prompt = fillSentencesPrompt({
      keyword: '  ',
      progress: 'Untouched',
      limit: 5,
      perSense: 1,
    })
    expect(prompt).toContain(
      '`noobenkyo dictionary export --completeness no-sentence --progress untouched --limit 5`',
    )
  })

  it('quotes a keyword the shell would split', () => {
    const prompt = fillSentencesPrompt({
      keyword: "it's お 茶",
      progress: 'Touched',
      limit: 20,
      perSense: 2,
    })
    expect(prompt).toContain(
      "`noobenkyo dictionary export --completeness no-sentence --keyword 'it'\\''s お 茶' --progress touched --limit 20`",
    )
  })
})
