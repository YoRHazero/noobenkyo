import { commands, type Language } from '@/bindings'
import { wavBlob } from '@/backend/listening/audio-bytes'
import { unwrap } from '@/backend/unwrap'

/**
 * A character's reference recording, to listen to; `null` if it has none.
 * Audio is large, so it is loaded when played and never cached as a query.
 */
export async function loadListeningReference(
  name: string,
  language: Language,
): Promise<Blob | null> {
  const reference = await unwrap(commands.listeningReference(name, language))
  return reference && wavBlob(reference.audio)
}
