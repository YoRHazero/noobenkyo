import { openTextFile } from '@/files/open-text-file'

const scriptFilters = [{ name: '剧本（JSON）', extensions: ['json'] }]

/** Asks for a script file and reads it as text; `null` if the user cancels. */
export function openScriptFile(): Promise<string | null> {
  return openTextFile(scriptFilters)
}
