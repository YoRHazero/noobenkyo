/** A span of the text, as offsets: `start` inclusive, `end` exclusive. */
export type TextRange = { start: number; end: number }

/**
 * Where a 1-based line sits in the text, numbered as the dictionary numbers
 * lines: split at '\n', blank lines counted, a trailing '\r' left out.
 * `null` past the last line.
 */
export function lineRange(text: string, line: number): TextRange | null {
  let start = 0
  for (let n = 1; n < line; n++) {
    const next = text.indexOf('\n', start)
    if (next === -1) {
      return null
    }
    start = next + 1
  }
  const newline = text.indexOf('\n', start)
  let end = newline === -1 ? text.length : newline
  if (end > start && text[end - 1] === '\r') {
    end -= 1
  }
  return { start, end }
}

/** The line as written; empty past the last line. */
export function lineText(text: string, line: number): string {
  const range = lineRange(text, line)
  return range ? text.slice(range.start, range.end) : ''
}
