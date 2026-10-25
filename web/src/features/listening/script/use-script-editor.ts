import { useState } from 'react'
import { editorStatus, type Editing } from '@/features/listening/script/script-editing'

/**
 * The preview's open editor. It lives with the script's source, above the
 * preview, so the source can tell whether an edit is still unwritten (when
 * the agent sends a new version) and close it when it loads another text.
 */
export function useScriptEditor() {
  const [editing, setEditing] = useState<Editing | null>(null)
  return { editing, setEditing, status: editorStatus(editing) }
}

export type ScriptEditor = ReturnType<typeof useScriptEditor>
