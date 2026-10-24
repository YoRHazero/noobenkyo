import type { BackdropProps } from '@/features/listening/theater/backdrop/backdrop-props'
import { nameHue } from '@/features/listening/name-hue'

/** One dark color for the whole world, its hue taken from the world's name. */
export function SolidBackdrop({ world }: BackdropProps) {
  return (
    <div
      className="absolute inset-0"
      style={{ backgroundColor: `oklch(0.22 0.04 ${nameHue(world)})` }}
    />
  )
}
