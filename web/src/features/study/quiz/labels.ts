import type { QuestionType, Rating } from '@/bindings'

export const questionTypeLabel: Record<QuestionType, string> = {
  Sentence: '例句题',
  Translation: '翻译题',
  Reading: '读音题',
  Comprehensive: '综合题',
}

export const ratingLabel: Record<Rating, string> = { Known: '会', Vague: '模糊', Unknown: '不会' }

/** The ratings in button order; keys 1, 2, 3 pick them. */
export const ratings: Rating[] = ['Known', 'Vague', 'Unknown']
