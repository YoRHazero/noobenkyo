import type { QuestionType, Rating, ReviewMode, Zone } from '@/bindings'

export const zoneLabel: Record<Zone, string> = { Learning: '学习区', Review: '复习区' }

export const questionTypeLabel: Record<QuestionType, string> = {
  Sentence: '例句题',
  Translation: '翻译题',
  Reading: '读音题',
  Comprehensive: '综合题',
}

export const ratingLabel: Record<Rating, string> = { Known: '会', Vague: '模糊', Unknown: '不会' }

export const reviewModeLabel: Record<ReviewMode, string> = {
  Endless: '无限模式',
  FixedGroup: '定组模式',
}
