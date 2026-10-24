import { commands, type NewCharacter } from '@/bindings'
import { byteArray } from '@/backend/listening/audio-bytes'
import type { UploadedReference } from '@/backend/listening/uploaded-reference'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

export type CharacterInput = Omit<NewCharacter, 'references'> & {
  references: UploadedReference[]
}

/** Resolves to the problems that rejected the character, empty when it was created. */
export function useListeningCreateCharacter() {
  return useChange(({ references, ...character }: CharacterInput) =>
    unwrap(
      commands.listeningCreateCharacter({
        ...character,
        references: references.map((reference) => ({
          ...reference,
          audio: byteArray(reference.audio),
        })),
      }),
    ),
  )
}
