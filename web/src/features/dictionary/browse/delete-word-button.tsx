import type { WordKey } from '@/bindings'
import { Trash2 } from 'lucide-react'
import { useDictionaryDeleteWord } from '@/backend/dictionary/use-dictionary-delete-word'
import { ConfirmButton } from '@/components/confirm-button'

/** Deletes a word with nothing below it, on the second click. */
export function DeleteWordButton({ word }: { word: WordKey }) {
  const deleteWord = useDictionaryDeleteWord()
  return (
    <ConfirmButton
      label={<Trash2 />}
      ariaLabel="删除单词"
      confirmLabel="确认删除"
      size="icon-sm"
      disabled={deleteWord.isPending}
      onConfirm={() => deleteWord.mutate(word)}
    />
  )
}
