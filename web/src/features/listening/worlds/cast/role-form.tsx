import { useState } from 'react'
import type { Role } from '@/bindings'
import { ChoiceButton } from '@/components/choice-button'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import {
  buildRole,
  dropsDescription,
  roleKinds,
  roleNames,
  type RoleKind,
} from '@/features/listening/worlds/cast/role-kinds'

type Props = {
  /** The role being changed, or `null` when adding someone. */
  initial: Role | null
  /** The kind chosen at first when adding someone. */
  startKind?: RoleKind
  submitLabel: string
  busy: boolean
  onSubmit: (role: Role) => void
  onCancel?: () => void
}

/**
 * A role and, for a main character, its description. When a main character
 * would become something else, the button says the description goes.
 */
export function RoleForm({ initial, startKind, submitLabel, busy, onSubmit, onCancel }: Props) {
  const [kind, setKind] = useState(initial?.kind ?? startKind ?? 'minor')
  const [description, setDescription] = useState(
    initial?.kind === 'main' ? initial.description : '',
  )
  return (
    <div className="flex flex-col gap-2">
      <div className="flex gap-1">
        {roleKinds.map((option) => (
          <ChoiceButton
            key={option}
            chosen={kind === option}
            disabled={busy}
            onClick={() => setKind(option)}
          >
            {roleNames[option]}
          </ChoiceButton>
        ))}
      </div>
      {kind === 'main' && (
        <Textarea
          aria-label="身份"
          placeholder="身份：性格、职业、经历、能力、人际关系"
          value={description}
          disabled={busy}
          className="min-h-20"
          onChange={(event) => setDescription(event.target.value)}
        />
      )}
      <div className="flex justify-end gap-2">
        {onCancel && (
          <Button variant="outline" size="sm" disabled={busy} onClick={onCancel}>
            取消
          </Button>
        )}
        <Button size="sm" disabled={busy} onClick={() => onSubmit(buildRole(kind, description))}>
          {submitLabel}
          {dropsDescription(initial, kind) && '（删除身份）'}
        </Button>
      </div>
    </div>
  )
}
