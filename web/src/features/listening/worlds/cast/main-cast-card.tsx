import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import { ChevronLeft } from 'lucide-react'
import type { CastMember, CharacterSummary } from '@/bindings'
import { Button } from '@/components/ui/button'
import { genderNames } from '@/features/listening/characters/gender-names'
import { CharacterPortrait } from '@/features/listening/portrait/character-portrait'
import { CastLanguages } from '@/features/listening/worlds/cast/cast-languages'
import { CastMemberButtons } from '@/features/listening/worlds/cast/cast-member-buttons'
import { CastRoleEditor } from '@/features/listening/worlds/cast/cast-role-editor'

type Props = {
  world: string
  member: CastMember
  character?: CharacterSummary
  expanded: boolean
  onToggle: () => void
}

/**
 * A main character as a tall card where its portrait goes. Folded, only the
 * card with the name; unfolded, it takes a row of its own with the reading,
 * gender, age and its description in this world.
 */
export function MainCastCard({ world, member, character, expanded, onToggle }: Props) {
  const [editing, setEditing] = useState(false)
  const card = (
    <button
      type="button"
      aria-expanded={expanded}
      aria-label={expanded ? `收起${member.character}` : `展开${member.character}`}
      className="relative aspect-[3/4] w-36 shrink-0 overflow-hidden rounded-xl text-left shadow-sm transition-transform hover:-translate-y-0.5"
      onClick={onToggle}
    >
      <CharacterPortrait name={member.character} shape="card" className="absolute inset-0" />
      <span className="absolute inset-x-0 bottom-0 flex items-end justify-between gap-1 bg-gradient-to-t from-black/70 to-transparent px-3 pt-8 pb-2.5 text-white">
        <span className="truncate font-medium">{member.character}</span>
        <CastLanguages languages={member.languages} className="text-white/70" />
      </span>
    </button>
  )
  if (!expanded) {
    return card
  }
  const role = member.role
  return (
    <div className="flex basis-full gap-4 rounded-2xl border bg-card p-3">
      {card}
      <div className="flex min-w-0 flex-1 flex-col gap-2 py-1">
        <div className="flex items-center gap-3">
          <Link
            to="/listening/characters/$name"
            params={{ name: member.character }}
            className="text-lg font-semibold underline-offset-4 hover:underline"
          >
            {member.character}
          </Link>
          {character && (
            <span className="text-sm text-muted-foreground">
              <span lang="ja">{character.kana}</span> · {genderNames[character.gender]} ·{' '}
              {character.age} 岁
            </span>
          )}
          <span className="ml-auto flex items-center">
            <CastMemberButtons world={world} member={member} onEdit={() => setEditing(!editing)} />
            <Button variant="ghost" size="icon-sm" aria-label="收起" onClick={onToggle}>
              <ChevronLeft />
            </Button>
          </span>
        </div>
        {editing ? (
          <CastRoleEditor world={world} member={member} onDone={() => setEditing(false)} />
        ) : (
          role.kind === 'main' && (
            <p className="text-sm whitespace-pre-wrap text-muted-foreground">{role.description}</p>
          )
        )}
      </div>
    </div>
  )
}
