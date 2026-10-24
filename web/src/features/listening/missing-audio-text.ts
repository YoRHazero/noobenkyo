/** 「缺音频：日语 3 行；中文 5 行」, leaving out what is complete; `null` when nothing is missing. */
export function missingAudioText(missingJa: number, missingZh: number): string | null {
  const parts = [
    missingJa > 0 ? `日语 ${missingJa} 行` : null,
    missingZh > 0 ? `中文 ${missingZh} 行` : null,
  ].filter((part) => part !== null)
  return parts.length === 0 ? null : `缺音频：${parts.join('；')}`
}
