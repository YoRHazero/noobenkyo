import { useEffect, useState } from 'react'

/**
 * Typed text once it has stayed the same for a moment, for the preview to
 * check; `settle` makes a text settled at once, for one the program put in.
 */
export function useSettledText(text: string) {
  const [settled, setSettled] = useState(text)
  useEffect(() => {
    const timer = setTimeout(() => setSettled(text), 250)
    return () => clearTimeout(timer)
  }, [text])
  return { settled, settle: setSettled }
}
