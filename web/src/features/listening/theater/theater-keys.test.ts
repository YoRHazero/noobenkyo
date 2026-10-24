import { describe, expect, it } from 'vitest'
import { theaterKey } from '@/features/listening/theater/theater-keys'

const press = (key: string, more: Partial<Parameters<typeof theaterKey>[0]> = {}) =>
  theaterKey({ key, altKey: false, ctrlKey: false, metaKey: false, isComposing: false, ...more })

describe('theaterKey', () => {
  it('maps the playing keys', () => {
    expect(
      [' ', 'ArrowRight', 'Enter', 'ArrowLeft', 'r', 'A', 's', 'Escape'].map((key) => press(key)),
    ).toEqual(['toggle', 'next', 'next', 'previous', 'replay', 'auto', 'reveal', 'close'])
  })

  it('leaves keys with modifiers alone, so copying works', () => {
    expect(press('c', { metaKey: true })).toBeNull()
    expect(press('a', { metaKey: true })).toBeNull()
    expect(press(' ', { isComposing: true })).toBeNull()
  })
})
