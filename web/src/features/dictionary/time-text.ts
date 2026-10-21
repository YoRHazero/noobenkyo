/** "2026-10-20 21:05", in local time. */
export function timeText(iso: string): string {
  const time = new Date(iso)
  const two = (n: number) => String(n).padStart(2, '0')
  return `${time.getFullYear()}-${two(time.getMonth() + 1)}-${two(time.getDate())} ${two(
    time.getHours(),
  )}:${two(time.getMinutes())}`
}
