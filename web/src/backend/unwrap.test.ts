import { describe, expect, it } from 'vitest'
import { unwrap } from '@/backend/unwrap'

describe('unwrap', () => {
  it('gives the data of a successful command', async () => {
    await expect(unwrap(Promise.resolve({ status: 'ok', data: 3 }))).resolves.toBe(3)
  })

  it('throws the message of a failed command as is', async () => {
    const failed = Promise.resolve({
      status: 'error' as const,
      error: { message: '当前没有题目' },
    })
    await expect(unwrap(failed)).rejects.toThrow(new Error('当前没有题目'))
  })
})
