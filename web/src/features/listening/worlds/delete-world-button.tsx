import { useListeningDeleteWorld } from '@/backend/listening/use-listening-delete-world'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'

type Props = {
  world: string
  episodes: number
  onDeleted: () => void
}

/** Deletes the world with its episodes on the second click, saying how many. */
export function DeleteWorldButton({ world, episodes, onDeleted }: Props) {
  const remove = useListeningDeleteWorld()
  return (
    <div className="flex items-center justify-end gap-3">
      <ErrorText error={remove.error} />
      <ConfirmButton
        label="删除世界"
        confirmLabel={`确认删除（${episodes} 集）`}
        disabled={remove.isPending}
        onConfirm={() => remove.mutate(world, { onSuccess: onDeleted })}
      />
    </div>
  )
}
