import type { CommandError } from '@/bindings'

/** What a command that can fail resolves to, as tauri-specta generates it. */
export type CommandResult<T> = { status: 'ok'; data: T } | { status: 'error'; error: CommandError }

/**
 * The data of a command result. Its error becomes a thrown `Error` carrying
 * the backend's message as is, which TanStack Query hands to the page.
 */
export async function unwrap<T>(pending: Promise<CommandResult<T>>): Promise<T> {
  const result = await pending
  if (result.status === 'error') {
    throw new Error(result.error.message)
  }
  return result.data
}
