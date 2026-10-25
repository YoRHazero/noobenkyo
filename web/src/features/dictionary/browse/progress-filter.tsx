import type { ProgressState } from '@/bindings'
import { ChoiceButton } from '@/components/choice-button'
import { progressStateName } from '@/features/dictionary/progress-state-names'

const states: ProgressState[] = ['Any', 'Untouched', 'Touched']

type Props = {
  value: ProgressState
  onChange: (value: ProgressState) => void
}

/** One of: any, untouched, or learned at all (learning or mastered). */
export function ProgressFilter({ value, onChange }: Props) {
  return (
    <div className="flex items-center gap-1 text-sm">
      <span className="text-muted-foreground">进度：</span>
      {states.map((state) => (
        <ChoiceButton key={state} chosen={value === state} onClick={() => onChange(state)}>
          {progressStateName[state]}
        </ChoiceButton>
      ))}
    </div>
  )
}
