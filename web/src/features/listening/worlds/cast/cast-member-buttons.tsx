import { useState } from 'react'
import { Pencil } from 'lucide-react'
import type { CastMember } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Dialog, DialogTrigger } from '@/components/ui/dialog'
import { RemoveCastDialog } from '@/features/listening/worlds/cast/remove-cast-dialog'

type Props = {
  world: string
  member: CastMember
  onEdit: () => void
}

/** ✎ opens the role editor; 移出 asks in a dialog, where it can still be cancelled. */
export function CastMemberButtons({ world, member, onEdit }: Props) {
  const [removing, setRemoving] = useState(false)
  return (
    <span className="inline-flex items-center gap-1">
      <Button variant="ghost" size="icon-sm" aria-label="改类型或身份" onClick={onEdit}>
        <Pencil />
      </Button>
      <Dialog open={removing} onOpenChange={setRemoving}>
        <DialogTrigger asChild>
          <Button variant="ghost" size="sm">
            移出
          </Button>
        </DialogTrigger>
        {removing && (
          <RemoveCastDialog world={world} member={member} onRemoved={() => setRemoving(false)} />
        )}
      </Dialog>
    </span>
  )
}
