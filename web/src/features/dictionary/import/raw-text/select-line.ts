import { lineRange } from '@/features/dictionary/import/line-range'

/**
 * Selects a line in the text box and scrolls it into view. The box does not
 * wrap lines, so line n sits n - 1 line heights from the top.
 */
export function selectLine(textarea: HTMLTextAreaElement, line: number) {
  const range = lineRange(textarea.value, line)
  if (!range) {
    return
  }
  textarea.focus()
  textarea.setSelectionRange(range.start, range.end)
  const lineHeight = parseFloat(getComputedStyle(textarea).lineHeight) || 24
  textarea.scrollTop = Math.max(0, (line - 1) * lineHeight - textarea.clientHeight / 3)
  textarea.scrollLeft = 0
}
