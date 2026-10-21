import type { SessionOptions } from '@/bindings'

/** A group of the most due senses, or all of them, endlessly. */
export type ReviewMode = 'group' | 'endless'

export const reviewModeLabel: Record<ReviewMode, string> = {
  group: '分组复习',
  endless: '无限脑力',
}

/**
 * The options a session starts with: the settings as they are for a group,
 * without a size for the endless mode. The settings page never stores an
 * empty size, so a group always has one.
 */
export function sessionOptions(settings: SessionOptions, mode: ReviewMode): SessionOptions {
  return mode === 'group' ? settings : { ...settings, size: null }
}
