import { useListeningDeleteEpisode } from '@/backend/listening/use-listening-delete-episode'
import { ConfirmButton } from '@/components/confirm-button'
import { ErrorText } from '@/components/error-text'

type Props = {
  episode: string
  onDeleted: () => void
}

export function DeleteEpisodeButton({ episode, onDeleted }: Props) {
  const remove = useListeningDeleteEpisode()
  return (
    <div className="flex items-center gap-3">
      <ErrorText error={remove.error} />
      <ConfirmButton
        label="删除这一集"
        confirmLabel="确认删除"
        disabled={remove.isPending}
        onConfirm={() => remove.mutate(episode, { onSuccess: onDeleted })}
      />
    </div>
  )
}
