type Result<T> = { status: 'ok'; data: T } | { status: 'error'; error: { message: string } }

/** Resolves a command result, throwing its error message so TanStack Query can show it. */
export async function unwrap<T>(pending: Promise<Result<T>>): Promise<T> {
  const result = await pending
  if (result.status === 'error') {
    throw new Error(result.error.message)
  }
  return result.data
}
