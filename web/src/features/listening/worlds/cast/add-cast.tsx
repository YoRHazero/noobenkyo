import { useState } from 'react'
import { Plus } from 'lucide-react'
import type { CastMember } from '@/bindings'
import { useListeningAddCast } from '@/backend/listening/use-listening-add-cast'
import { useListeningCharacters } from '@/backend/listening/use-listening-characters'
import { ErrorText } from '@/components/error-text'
import { cn } from '@/components/ui/cn'
import { RoleForm } from '@/features/listening/worlds/cast/role-form'
import { roleNames, type RoleKind } from '@/features/listening/worlds/cast/role-kinds'

type Props = {
  world: string
  cast: CastMember[]
  /** The section it sits in, chosen at first in the form. */
  kind: RoleKind
  /** The empty seat as the section draws its members: a strip, a card or a chip. */
  look: 'strip' | 'card' | 'chip'
}

const seat: Record<Props['look'], string> = {
  strip: 'h-12 w-full justify-center',
  card: 'aspect-[3/4] w-36 flex-col justify-center',
  chip: 'h-10 px-4',
}

/** An empty seat in a section of the cast: it opens the form for casting someone there. */
export function AddCast({ world, cast, kind, look }: Props) {
  const [open, setOpen] = useState(false)
  const [character, setCharacter] = useState('')
  const characters = useListeningCharacters()
  const add = useListeningAddCast()
  const candidates = (characters.data ?? [])
    .map((c) => c.name)
    .filter((name) => !cast.some((member) => member.character === name))
  const chosen = candidates.includes(character) ? character : (candidates[0] ?? '')

  if (!open) {
    return (
      <button
        type="button"
        disabled={candidates.length === 0}
        className={cn(
          'flex items-center gap-1.5 rounded-xl border border-dashed text-sm text-muted-foreground transition-colors hover:border-foreground/40 hover:text-foreground disabled:pointer-events-none disabled:opacity-50',
          look === 'chip' && 'rounded-full',
          seat[look],
        )}
        onClick={() => setOpen(true)}
      >
        <Plus className="size-4" />
        {candidates.length === 0 ? '无可添加的角色' : roleNames[kind]}
      </button>
    )
  }
  return (
    <div className="flex basis-full flex-col gap-2 rounded-xl border border-dashed px-4 py-3">
      <select
        aria-label="角色"
        value={chosen}
        disabled={add.isPending}
        className="h-8 w-48 rounded-md border border-input bg-transparent px-2 text-sm"
        onChange={(event) => setCharacter(event.target.value)}
      >
        {candidates.map((name) => (
          <option key={name} value={name}>
            {name}
          </option>
        ))}
      </select>
      <RoleForm
        initial={null}
        startKind={kind}
        submitLabel="添加"
        busy={add.isPending}
        onCancel={() => setOpen(false)}
        onSubmit={(role) =>
          add.mutate({ world, character: chosen, role }, { onSuccess: () => setOpen(false) })
        }
      />
      <ErrorText error={add.error} />
    </div>
  )
}
