import type { LearningOrder, QuestionType } from '@/bindings'

export const questionTypeLabel: Record<QuestionType, string> = {
  Sentence: '例句题',
  Translation: '翻译题',
  Reading: '读音题',
  Comprehensive: '综合题',
}

export const learningOrderLabel: Record<LearningOrder, string> = {
  Creation: '按建立顺序',
  Random: '随机',
}
