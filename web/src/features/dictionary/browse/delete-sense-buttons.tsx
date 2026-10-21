import type { SenseDetail } from '@/bindings'
import { useDictionaryDeleteReading } from '@/backend/dictionary/use-dictionary-delete-reading'
import { useDictionaryDeleteSense } from '@/backend/dictionary/use-dictionary-delete-sense'
import { useDictionaryDeleteWord } from '@/backend/dictionary/use-dictionary-delete-word'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'

/**
 * Deleting the sense, its reading or its word, each on the second click,
 * which names what goes with it.
 */
export function DeleteSenseButtons({ detail }: { detail: SenseDetail }) {
  const deleteSense = useDictionaryDeleteSense()
  const deleteReading = useDictionaryDeleteReading()
  const deleteWord = useDictionaryDeleteWord()
  const busy = deleteSense.isPending || deleteReading.isPending || deleteWord.isPending
  const { key } = detail
  return (
    <>
      <ConfirmButton
        label="删除义项"
        confirmLabel="确认删除"
        disabled={busy}
        onConfirm={() => deleteSense.mutate(key)}
      />
      <ConfirmButton
        label="删除读音"
        confirmLabel={`确认删除（含 ${detail.reading_senses} 个义项）`}
        disabled={busy}
        onConfirm={() =>
          deleteReading.mutate({ writing: key.writing, origin: key.origin, kana: key.kana })
        }
      />
      <ConfirmButton
        label="删除单词"
        confirmLabel={`确认删除（含 ${detail.word_readings} 个读音、${detail.word_senses} 个义项）`}
        disabled={busy}
        onConfirm={() => deleteWord.mutate({ writing: key.writing, origin: key.origin })}
      />
      <ErrorText error={deleteSense.error ?? deleteReading.error ?? deleteWord.error} />
    </>
  )
}
