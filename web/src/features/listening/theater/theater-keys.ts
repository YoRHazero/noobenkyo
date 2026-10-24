/** What a key does in the theater. */
export type TheaterKey = 'toggle' | 'next' | 'previous' | 'replay' | 'auto' | 'reveal' | 'close'

type Pressed = {
  key: string
  altKey: boolean
  ctrlKey: boolean
  metaKey: boolean
  isComposing: boolean
}

/**
 * Single keys without modifiers only, so Cmd+C still copies the text that
 * was selected. Escape only closes a panel.
 */
export function theaterKey({
  key,
  altKey,
  ctrlKey,
  metaKey,
  isComposing,
}: Pressed): TheaterKey | null {
  if (altKey || ctrlKey || metaKey || isComposing) {
    return null
  }
  switch (key) {
    case ' ':
      return 'toggle'
    case 'ArrowRight':
    case 'Enter':
      return 'next'
    case 'ArrowLeft':
      return 'previous'
    case 'r':
    case 'R':
      return 'replay'
    case 'a':
    case 'A':
      return 'auto'
    case 's':
    case 'S':
      return 'reveal'
    case 'Escape':
      return 'close'
    default:
      return null
  }
}
