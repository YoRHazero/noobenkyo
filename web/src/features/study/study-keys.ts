import type { Zone } from '@/bindings'

/** Query keys of the study pages. */
export const studyKeys = {
  all: ['study'] as const,
  zoneStatus: (zone: Zone) => ['study', 'zone-status', zone] as const,
  question: (zone: Zone, round: number) => ['study', 'question', zone, round] as const,
}
