import { cn } from '@/components/ui/cn'
import { nameHue } from '@/features/listening/name-hue'

type Props = {
  name: string
  /** card: the tall picture a portrait fills; avatar: a round face. */
  shape: 'card' | 'avatar'
  className?: string
}

/**
 * What a character looks like. Characters have no portrait yet, so this is
 * the stand-in: the character's color, the same as its name plate in the
 * theater, with the first character of its name where the portrait will be.
 * Everything that shows a character goes through here, so adding portraits
 * changes only this component.
 */
export function CharacterPortrait({ name, shape, className }: Props) {
  const hue = nameHue(name)
  const initial = [...name][0] ?? ''
  if (shape === 'avatar') {
    return (
      <span
        aria-hidden
        className={cn(
          'flex size-8 shrink-0 items-center justify-center rounded-full text-sm font-medium text-white',
          className,
        )}
        style={{ backgroundColor: `oklch(0.55 0.12 ${hue})` }}
      >
        {initial}
      </span>
    )
  }
  return (
    <div
      aria-hidden
      className={cn('flex items-center justify-center overflow-hidden', className)}
      style={{
        backgroundImage: `linear-gradient(160deg, oklch(0.62 0.11 ${hue}), oklch(0.36 0.09 ${hue}))`,
      }}
    >
      <span className="text-7xl font-semibold text-white/25 select-none">{initial}</span>
    </div>
  )
}
