import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import type { CastMember, CharacterSummary } from '@/bindings'
import { CastLanguages } from '@/features/listening/worlds/cast/cast-languages'
import { CastMemberButtons } from '@/features/listening/worlds/cast/cast-member-buttons'
import { CastRoleEditor } from '@/features/listening/worlds/cast/cast-role-editor'

type Props = {
  world: string
  member: CastMember
  character?: CharacterSummary
}

/** The narrator: a single strip, no picture, since the narrator is only a voice. */
export function NarratorStrip({ world, member, character }: Props) {
  const [editing, setEditing] = useState(false)
  return (
    <div className="flex flex-col gap-2 rounded-xl border bg-card px-4 py-2.5">
      <div className="flex items-center gap-3 text-sm">
        <Link
          to="/listening/characters/$name"
          params={{ name: member.character }}
          className="font-medium underline-offset-4 hover:underline"
        >
          {member.character}
        </Link>
        {character && (
          <span lang="ja" className="text-muted-foreground">
            {character.kana}
          </span>
        )}
        <CastLanguages languages={member.languages} className="text-muted-foreground" />
        <span className="ml-auto">
          <CastMemberButtons world={world} member={member} onEdit={() => setEditing(!editing)} />
        </span>
      </div>
      {editing && <CastRoleEditor world={world} member={member} onDone={() => setEditing(false)} />}
    </div>
  )
}
