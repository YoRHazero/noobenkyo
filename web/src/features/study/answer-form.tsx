import type { AnswerInput } from '@/bindings'
import { Button } from '@/components/ui/button'
import { ChineseInput } from '@/features/study/chinese-input'
import { KanaInput } from '@/features/study/kana-input'

type Props = {
  input: AnswerInput
  pending: boolean
  onSubmit: (answer: string) => void
}

/** The answer box a question asks for; Enter submits. */
export function AnswerForm({ input, pending, onSubmit }: Props) {
  switch (input.kind) {
    case 'chinese':
      return <ChineseInput disabled={pending} onSubmit={onSubmit} />
    case 'kana':
      return <KanaInput maxLength={input.max_length} disabled={pending} onSubmit={onSubmit} />
    case 'none':
      return (
        <Button autoFocus disabled={pending} onClick={() => onSubmit('')}>
          显示答案
        </Button>
      )
  }
}
