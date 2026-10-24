import { describe, expect, it } from 'vitest'
import { buildRole, dropsDescription } from '@/features/listening/worlds/cast/role-kinds'

describe('roles', () => {
  it('keeps a description only for a main character', () => {
    expect(buildRole('main', '高中生。')).toEqual({ kind: 'main', description: '高中生。' })
    expect(buildRole('minor', '高中生。')).toEqual({ kind: 'minor' })
  })

  it('warns only when a main character stops being one', () => {
    const main = { kind: 'main', description: '高中生。' } as const
    expect(dropsDescription(main, 'minor')).toBe(true)
    expect(dropsDescription(main, 'main')).toBe(false)
    expect(dropsDescription({ kind: 'minor' }, 'narrator')).toBe(false)
    expect(dropsDescription(null, 'minor')).toBe(false)
  })
})
