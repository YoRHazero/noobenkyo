import type { ProgressState } from '@/bindings'
import { ChoiceButton } from '@/features/dictionary/browse/choice-button'

const states: { state: ProgressState; label: string }[] = [
  { state: 'Any', label: '不限' },
  { state: 'Untouched', label: '未学习' },
  { state: 'Touched', label: '已学习' },
]

type Props = {
  value: ProgressState
  onChange: (value: ProgressState) => void
}

/** One of: any, untouched, or learned at all (learning or mastered). */
export function ProgressFilter({ value, onChange }: Props) {
  return (
    <div className="flex items-center gap-1 text-sm">
      <span className="text-muted-foreground">进度：</span>
      {states.map(({ state, label }) => (
        <ChoiceButton key={state} chosen={value === state} onClick={() => onChange(state)}>
          {label}
        </ChoiceButton>
      ))}
    </div>
  )
}
