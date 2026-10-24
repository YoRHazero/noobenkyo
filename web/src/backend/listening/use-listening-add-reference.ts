import { commands } from '@/bindings'
import { byteArray } from '@/backend/listening/audio-bytes'
import type { UploadedReference } from '@/backend/listening/uploaded-reference'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = UploadedReference & {
  name: string
}

export function useListeningAddReference() {
  return useChange(({ name, language, audio, text }: Input) =>
    unwrap(commands.listeningAddReference(name, language, byteArray(audio), text)),
  )
}
