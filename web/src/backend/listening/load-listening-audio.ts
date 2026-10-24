import { commands, type Language, type LineId } from '@/bindings'
import { wavBlob } from '@/backend/listening/audio-bytes'
import { unwrap } from '@/backend/unwrap'

/**
 * A line's audio in one language, to play; `null` if it has none. Audio is
 * large, so it is loaded when played and never cached as a query.
 */
export async function loadListeningAudio(line: LineId, language: Language): Promise<Blob | null> {
  const audio = await unwrap(commands.listeningAudio(line, language))
  return audio && wavBlob(audio)
}
