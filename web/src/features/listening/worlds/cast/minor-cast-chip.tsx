import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import type { CastMember } from '@/bindings'
import { cn } from '@/components/ui/cn'
import { CharacterPortrait } from '@/features/listening/portrait/character-portrait'
import { CastLanguages } from '@/features/listening/worlds/cast/cast-languages'
import { CastMemberButtons } from '@/features/listening/worlds/cast/cast-member-buttons'
import { CastRoleEditor } from '@/features/listening/worlds/cast/cast-role-editor'

type Props = {
  world: string
  member: CastMember
}

/** A minor character, only a voice here: a small round face and the name. */
export function MinorCastChip({ world, member }: Props) {
  const [editing, setEditing] = useState(false)
  return (
    <div
      className={cn(
        'flex flex-col gap-2 rounded-full border bg-card py-1 pr-1 pl-1',
        editing && 'basis-full rounded-xl px-3 py-2',
      )}
    >
      <div className="flex items-center gap-2 text-sm">
        <CharacterPortrait name={member.character} shape="avatar" />
        <Link
          to="/listening/characters/$name"
          params={{ name: member.character }}
          className="font-medium underline-offset-4 hover:underline"
        >
          {member.character}
        </Link>
        <CastLanguages languages={member.languages} className="text-muted-foreground" />
        <CastMemberButtons world={world} member={member} onEdit={() => setEditing(!editing)} />
      </div>
      {editing && <CastRoleEditor world={world} member={member} onDone={() => setEditing(false)} />}
    </div>
  )
}
