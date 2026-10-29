/**
 * Audio crosses the IPC as JSON: bytes become a `number[]`, several times
 * their size in memory. These turn it back into compact bytes as soon as it
 * arrives, and bytes read from a file into the array a command takes. Only
 * this file knows the format, so it alone changes if raw bytes are used.
 */

/** A WAV recording from a command, ready for an `<audio>` element. */
export function wavBlob(bytes: number[]): Blob {
  return new Blob([Uint8Array.from(bytes)], { type: 'audio/wav' })
}

/** A WAV recording from a command, as bytes to keep until it is stored. */
export function wavBytes(bytes: number[]): Uint8Array {
  return Uint8Array.from(bytes)
}

/** Bytes read from a file, as a command takes them. */
export function byteArray(bytes: Uint8Array): number[] {
  return Array.from(bytes)
}
