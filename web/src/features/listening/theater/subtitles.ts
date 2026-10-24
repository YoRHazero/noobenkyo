import type { Language } from '@/bindings'

/** Which languages are shown in the dialogue box; hidden ones can be revealed for one line. */
export type Subtitles = Record<Language, boolean>

export const allSubtitles: Subtitles = { ja: true, zh: true }

/** The narrator's lines are written as 「旁白」 in every script: they get no name plate. */
export const narratorSpeaker = '旁白'
