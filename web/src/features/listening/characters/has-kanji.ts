/**
 * Whether a name has kanji, by the same ranges listening uses: CJK
 * ideographs plus 々, 〆 and 〇. Only to tell the form that a name without
 * kanji is its own reading; listening still checks the name itself.
 */
export function hasKanji(text: string): boolean {
  return /[㐀-䶿一-鿿豈-﫿\u{20000}-\u{2FA1F}々〆〇]/u.test(text)
}
