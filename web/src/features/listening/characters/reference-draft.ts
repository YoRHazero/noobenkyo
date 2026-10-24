import type { BinaryFile } from '@/files/open-binary-file'

/** A reference being added: the chosen file, if any, and what is said in it. */
export type ReferenceDraft = {
  file: BinaryFile | null
  text: string
}

export const emptyReference: ReferenceDraft = { file: null, text: '' }
