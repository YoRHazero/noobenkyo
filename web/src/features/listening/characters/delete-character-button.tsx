import { useListeningCharacterUsage } from '@/backend/listening/use-listening-character-usage'
import { useListeningDeleteCharacter } from '@/backend/listening/use-listening-delete-character'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'

type Props = {
  name: string
  onDeleted: () => void
}

/** Deletes the character on the second click, saying how many casts lose it. */
export function DeleteCharacterButton({ name, onDeleted }: Props) {
  const usage = useListeningCharacterUsage(name)
  const remove = useListeningDeleteCharacter()
  const worlds = usage.data?.length ?? 0
  return (
    <div className="flex items-center justify-end gap-3">
      <ErrorText error={remove.error} />
      <ConfirmButton
        label="删除角色"
        confirmLabel={worlds > 0 ? `确认删除（移出 ${worlds} 个世界）` : '确认删除'}
        disabled={remove.isPending || usage.isPending}
        onConfirm={() => remove.mutate(name, { onSuccess: onDeleted })}
      />
    </div>
  )
}
