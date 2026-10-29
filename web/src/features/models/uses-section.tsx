import type { TtsOp } from '@/bindings'
import { modelOptions } from '@/features/models/model-options'
import type { ModelsDraft } from '@/features/models/models-draft'
import { UseRow } from '@/features/models/use-row'

type Props = {
  draft: ModelsDraft
  onDraft: (draft: ModelsDraft) => void
}

type Use = {
  label: string
  op: TtsOp
  value: (draft: ModelsDraft) => string | null
  set: (draft: ModelsDraft, model: string | null) => ModelsDraft
}

const uses: Use[] = [
  {
    label: '合成台词',
    op: 'TtsClone',
    value: (draft) => draft.speak,
    set: (draft, speak) => ({ ...draft, speak }),
  },
  {
    label: '生成参考音',
    op: 'TtsDesign',
    value: (draft) => draft.designVoice,
    set: (draft, designVoice) => ({ ...draft, designVoice }),
  },
]

/** The model each use runs on, from those ticked on the machines below. */
export function UsesSection({ draft, onDraft }: Props) {
  return (
    <section className="flex flex-col">
      <h2 className="mb-1 font-semibold">用途</h2>
      <div className="flex flex-col divide-y">
        {uses.map((use) => (
          <UseRow
            key={use.label}
            label={use.label}
            op={use.op}
            options={modelOptions(draft.machines, use.op)}
            value={use.value(draft)}
            onChange={(model) => onDraft(use.set(draft, model))}
          />
        ))}
      </div>
    </section>
  )
}
