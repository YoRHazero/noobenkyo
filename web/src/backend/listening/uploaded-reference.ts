import type { Language } from '@/bindings'

/** A reference recording as read from the user's file, before it crosses the IPC. */
export type UploadedReference = {
  language: Language
  audio: Uint8Array
  text: string
}
