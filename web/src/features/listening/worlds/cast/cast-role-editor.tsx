import type { CastMember } from '@/bindings'
import { useListeningSetRole } from '@/backend/listening/use-listening-set-role'
import { ErrorText } from '@/components/error-text'
import { RoleForm } from '@/features/listening/worlds/cast/role-form'

type Props = {
  world: string
  member: CastMember
  onDone: () => void
}

/** Changes a cast member's kind of role, or a main character's description. */
export function CastRoleEditor({ world, member, onDone }: Props) {
  const setRole = useListeningSetRole()
  return (
    <div className="flex flex-col gap-2">
      <RoleForm
        initial={member.role}
        submitLabel="保存"
        busy={setRole.isPending}
        onCancel={onDone}
        onSubmit={(role) =>
          setRole.mutate({ world, character: member.character, role }, { onSuccess: onDone })
        }
      />
      <ErrorText error={setRole.error} />
    </div>
  )
}
