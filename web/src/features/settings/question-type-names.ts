import type { QuestionType } from '@/bindings'

/** As the question orders write them, without "题". */
export const questionTypeName: Record<QuestionType, string> = {
  Sentence: '例句',
  Translation: '翻译',
  Reading: '读音',
  Comprehensive: '综合',
}
