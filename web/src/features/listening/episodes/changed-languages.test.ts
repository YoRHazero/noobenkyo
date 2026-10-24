import { describe, expect, it } from 'vitest'
import type { LineView } from '@/bindings'
import { changedLanguages, saveLabel } from '@/features/listening/episodes/changed-languages'

const line: LineView = {
  id: 1,
  speaker: '美咲',
  voice: '美咲',
  ja: '{灯台|とうだい}へ。',
  ja_ruby: [],
  zh: '去灯塔。',
  audio: ['ja'],
}

describe('changed languages', () => {
  it('ignores what only differs in surrounding blanks', () => {
    expect(changedLanguages(line, { ja: ' {灯台|とうだい}へ。\n', zh: '去灯塔。' })).toEqual([])
    expect(changedLanguages(line, { ja: '{灯台|とうだい}へ！', zh: '去灯塔吧。' })).toEqual([
      'ja',
      'zh',
    ])
  })

  it('says which audio saving deletes', () => {
    expect(saveLabel(line, ['ja', 'zh'])).toBe('保存（删除日语音频）')
    expect(saveLabel(line, ['zh'])).toBe('保存')
  })
})
