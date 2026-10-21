import type { AnswerInput } from '@/bindings'
import { Button } from '@/components/ui/button'
import { ChineseInput } from '@/features/study/quiz/chinese-input'
import { KanaInput } from '@/features/study/quiz/kana-input'

type Props = {
  answer: AnswerInput
  disabled: boolean
  onSubmit: (input: string) => void
}

/** The answer box the question asks for; without one, only a button to reveal. */
export function AnswerForm({ answer, disabled, onSubmit }: Props) {
  switch (answer.kind) {
    case 'chinese':
      return <ChineseInput disabled={disabled} onSubmit={onSubmit} />
    case 'kana':
      return <KanaInput maxLength={answer.max_length} disabled={disabled} onSubmit={onSubmit} />
    case 'none':
      return (
        <Button autoFocus className="self-start" disabled={disabled} onClick={() => onSubmit('')}>
          显示答案
        </Button>
      )
  }
}
