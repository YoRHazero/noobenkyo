import { openTextFile } from '@/files/open-text-file'
import { openFilters } from '@/features/dictionary/entry-file/entry-file-filters'

/** Asks for an entry file and reads it as text; `null` if the user cancels. */
export function openEntryFile(): Promise<string | null> {
  return openTextFile(openFilters)
}
