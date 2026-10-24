const digits = ['', '一', '二', '三', '四', '五', '六', '七', '八', '九']

/** An age from 1 to 100 in Chinese numerals, as a description writes it: 17 → 十七. */
export function ageText(age: number): string {
  if (age === 100) {
    return '一百'
  }
  const tens = Math.floor(age / 10)
  const ones = digits[age % 10] ?? ''
  if (tens === 0) {
    return ones
  }
  return `${tens === 1 ? '' : digits[tens]}十${ones}`
}
