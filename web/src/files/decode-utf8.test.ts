import { describe, expect, it } from 'vitest'
import { decodeUtf8 } from '@/files/decode-utf8'

const bytes = (...values: number[]) => new Uint8Array(values)

describe('decodeUtf8', () => {
  it('decodes UTF-8', () => {
    expect(decodeUtf8(new TextEncoder().encode('{"writing": "明日"}'))).toBe('{"writing": "明日"}')
  })

  it('drops a leading BOM', () => {
    expect(decodeUtf8(bytes(0xef, 0xbb, 0xbf, 0x61))).toBe('a')
  })

  it('refuses other encodings', () => {
    // 明日 in Shift_JIS.
    expect(() => decodeUtf8(bytes(0x96, 0xbe, 0x93, 0xfa))).toThrow('文件不是 UTF-8 编码')
  })
})
