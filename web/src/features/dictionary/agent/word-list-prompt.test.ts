import { describe, expect, it } from 'vitest'
import { wordListPrompt } from '@/features/dictionary/agent/word-list-prompt'

describe('wordListPrompt', () => {
  it('gives the topic and counts', () => {
    const prompt = wordListPrompt({ topic: '便利店购物', count: 50, perSense: 2 })
    expect(prompt)
      .toBe(`用 noobenkyo 命令行生成一份日语词表，送进 app。用法先运行 \`noobenkyo dictionary guide\` 看。

话题：便利店购物
个数：50
只列动词、形容词、副词和名词，写到例句，每个义项 2 条例句。词库里已有的词不要。`)
  })

  it('leaves a blank topic for the agent to choose', () => {
    const prompt = wordListPrompt({ topic: ' ', count: 10, perSense: 1 })
    expect(prompt).toContain('话题：没有指定，自己选一个日常的话题\n')
  })
})
