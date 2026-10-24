import type { CastMember } from '@/bindings'
import { useListeningRemoveCast } from '@/backend/listening/use-listening-remove-cast'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import {
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'

type Props = {
  world: string
  member: CastMember
  onRemoved: () => void
}

/**
 * Asks before taking a character out of the world, saying what goes with it:
 * a main character's description. Episodes already made keep their voices.
 */
export function RemoveCastDialog({ world, member, onRemoved }: Props) {
  const remove = useListeningRemoveCast()
  const busy = remove.isPending
  const keepOpen = (event: Event) => {
    if (busy) {
      event.preventDefault()
    }
  }
  return (
    <DialogContent
      aria-describedby={undefined}
      showCloseButton={!busy}
      onEscapeKeyDown={keepOpen}
      onInteractOutside={keepOpen}
      className="sm:max-w-sm"
    >
      <DialogHeader>
        <DialogTitle>移出角色</DialogTitle>
      </DialogHeader>
      <div className="flex flex-col gap-1 text-sm">
        <p>角色：{member.character}</p>
        <p>世界：{world}</p>
        {member.role.kind === 'main' && <p className="text-destructive">身份会删除</p>}
        <p className="text-muted-foreground">已经生成的集不受影响</p>
      </div>
      <ErrorText error={remove.error} />
      <DialogFooter>
        <DialogClose asChild>
          <Button variant="outline" disabled={busy}>
            取消
          </Button>
        </DialogClose>
        <Button
          variant="destructive"
          disabled={busy}
          onClick={() =>
            remove.mutate({ world, character: member.character }, { onSuccess: onRemoved })
          }
        >
          移出
        </Button>
      </DialogFooter>
    </DialogContent>
  )
}
