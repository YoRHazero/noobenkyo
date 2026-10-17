import { useEffect, useState } from 'react'
import { events, type ImportProgress } from '@/bindings'

/** The latest import progress pushed by the backend while `active`. */
export function useImportProgress(active: boolean): ImportProgress | null {
  const [progress, setProgress] = useState<ImportProgress | null>(null)
  useEffect(() => {
    if (!active) {
      return
    }
    const stop = events.importProgressEvent.listen((event) => setProgress(event.payload))
    return () => {
      void stop.then((unlisten) => unlisten())
    }
  }, [active])
  return active ? progress : null
}
