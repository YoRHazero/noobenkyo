/**
 * A hue from 0 to 359 for a name, always the same for the same name: worlds
 * and voices get their colors from it without storing any.
 */
export function nameHue(name: string): number {
  let hash = 0
  for (const char of name) {
    hash = (hash * 31 + (char.codePointAt(0) ?? 0)) >>> 0
  }
  return hash % 360
}
