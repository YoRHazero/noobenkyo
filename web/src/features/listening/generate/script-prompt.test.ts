import { describe, expect, it } from 'vitest'
import { scriptPrompt } from '@/features/listening/generate/script-prompt'

describe('scriptPrompt', () => {
  it('lists the chosen episodes in order for a new episode', () => {
    const prompt = scriptPrompt({
      world: '星见町',
      appending: null,
      previous: ['9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b', '4f2a6b7c-8d9e-4f0a-9b1c-2d3e4f5a6b7c'],
      premise: '  美咲和健二在祭典上走散。\n最后在神社的台阶上遇到。\n',
    })
    expect(prompt)
      .toBe(`用 noobenkyo 命令行给听力的世界「星见町」写新的一集剧本，送进 app。用法先运行 \`noobenkyo listening guide\` 看。

引子：
美咲和健二在祭典上走散。
最后在神社的台阶上遇到。

前序集，按这个顺序：
9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b
4f2a6b7c-8d9e-4f0a-9b1c-2d3e4f5a6b7c`)
  })

  it('says what is left open is the agent’s to judge', () => {
    const prompt = scriptPrompt({
      world: '星见町',
      appending: '9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b',
      previous: [],
      premise: ' \n',
    })
    expect(prompt)
      .toBe(`用 noobenkyo 命令行续写听力的世界「星见町」里的一集（9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b），送进 app。用法先运行 \`noobenkyo listening guide\` 看。

引子：没有指定，自己按世界设定和之前的剧情想一个。

前序集：没有指定，自己判断要参考这个世界里的哪些集。`)
  })
})
