import { describe, expect, it } from 'vitest'
import { nameHue } from '@/features/listening/name-hue'

describe('nameHue', () => {
  it('gives a name the same hue every time', () => {
    expect(nameHue('星见町')).toBe(nameHue('星见町'))
  })

  it('stays within the color wheel and tells names apart', () => {
    const hues = ['美咲', '健二', '幸子', '田中'].map(nameHue)
    expect(hues.every((hue) => hue >= 0 && hue < 360)).toBe(true)
    expect(new Set(hues).size).toBe(4)
  })
})
