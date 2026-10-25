/**
 * Puts text on the clipboard. Call it straight from a click: the webview
 * only lets a page write the clipboard while handling the user's action.
 */
export async function writeClipboard(text: string): Promise<void> {
  await navigator.clipboard.writeText(text).catch((error: unknown) => {
    throw new Error(`写入剪贴板失败：${String(error)}`)
  })
}
