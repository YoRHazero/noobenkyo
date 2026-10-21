/**
 * Takes the given 1-based lines out of the text, numbered as the dictionary
 * numbers them. The cut lines keep their order, without a trailing '\r'.
 * What is left ends with a line break, so whatever is pasted at its end
 * starts on a line of its own.
 */
export function cutLines(text: string, lines: number[]): { cut: string[]; rest: string } {
  const chosen = new Set(lines)
  const cut: string[] = []
  const kept: string[] = []
  text.split('\n').forEach((line, index) => {
    if (chosen.has(index + 1)) {
      cut.push(line.replace(/\r$/, ''))
    } else {
      kept.push(line)
    }
  })
  const rest = kept.join('\n')
  return { cut, rest: rest === '' || rest.endsWith('\n') ? rest : `${rest}\n` }
}
