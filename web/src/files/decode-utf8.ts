/**
 * The text of a file's bytes. Anything but UTF-8 is refused rather than
 * decoded with replacement characters, which would import garbled glosses.
 * A leading BOM is dropped.
 */
export function decodeUtf8(bytes: Uint8Array): string {
  try {
    return new TextDecoder('utf-8', { fatal: true }).decode(bytes)
  } catch {
    throw new Error('文件不是 UTF-8 编码')
  }
}
