/** Characters a shell would split a word at, or expand. */
const special = /[\s'"\\$`!*?[\](){}<>|&;#~=%]/

/**
 * `text` as one word of a shell command: as it is when the shell would
 * leave it alone, otherwise in single quotes.
 */
export function shellWord(text: string): string {
  if (text !== '' && !special.test(text)) {
    return text
  }
  // A single quote cannot appear inside single quotes: close, escape, reopen.
  return `'${text.replaceAll("'", "'\\''")}'`
}
