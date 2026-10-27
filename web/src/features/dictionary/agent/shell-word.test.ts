import { describe, expect, it } from 'vitest'
import { shellWord } from '@/features/dictionary/agent/shell-word'

describe('shellWord', () => {
  it('leaves plain words as they are', () => {
    expect(shellWord('猫')).toBe('猫')
    expect(shellWord('食べる')).toBe('食べる')
  })

  it('quotes words the shell would split or expand', () => {
    expect(shellWord('お 茶')).toBe("'お 茶'")
    expect(shellWord('$HOME')).toBe("'$HOME'")
    expect(shellWord("it's")).toBe("'it'\\''s'")
    expect(shellWord('')).toBe("''")
  })
})
