/**
 * The dialog and fs plugins fail with plain strings, which pages cannot show.
 * Rethrows one as an `Error` saying what was being done.
 */
export function pluginError(action: string): (error: unknown) => never {
  return (error) => {
    throw new Error(`${action}失败：${String(error)}`)
  }
}
