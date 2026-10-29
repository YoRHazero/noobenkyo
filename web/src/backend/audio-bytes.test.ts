import { describe, expect, it } from 'vitest'
import { byteArray, wavBlob, wavBytes } from '@/backend/audio-bytes'

describe('audio bytes', () => {
  it('turns a command result into a WAV blob with the same bytes', async () => {
    const blob = wavBlob([82, 73, 70, 70, 0, 255])
    expect(blob.type).toBe('audio/wav')
    expect([...new Uint8Array(await blob.arrayBuffer())]).toEqual([82, 73, 70, 70, 0, 255])
  })

  it('turns a command result into compact bytes', () => {
    expect(wavBytes([82, 73, 0, 255])).toEqual(new Uint8Array([82, 73, 0, 255]))
  })

  it('turns bytes from a file into the array a command takes', () => {
    expect(byteArray(new Uint8Array([0, 128, 255]))).toEqual([0, 128, 255])
  })
})
