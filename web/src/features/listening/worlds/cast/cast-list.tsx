import { useState, type ReactNode } from 'react'
import type { CastMember } from '@/bindings'
import { useListeningCharacters } from '@/backend/listening/use-listening-characters'
import { AddCast } from '@/features/listening/worlds/cast/add-cast'
import { MainCastCard } from '@/features/listening/worlds/cast/main-cast-card'
import { MinorCastChip } from '@/features/listening/worlds/cast/minor-cast-chip'
import { NarratorStrip } from '@/features/listening/worlds/cast/narrator-strip'

type Props = {
  world: string
  /** As listening orders it: the narrator, main characters, minor ones. */
  cast: CastMember[]
}

/**
 * The cast as a playbill: the narrator's strip, the main characters' cards,
 * the minor characters' chips. Each section ends in an empty seat that casts
 * someone straight into it.
 */
export function CastList({ world, cast }: Props) {
  const characters = useListeningCharacters()
  const [expanded, setExpanded] = useState<ReadonlySet<string>>(new Set())
  const profile = (name: string) => characters.data?.find((c) => c.name === name)
  const narrator = cast.find((member) => member.role.kind === 'narrator')
  const mains = cast.filter((member) => member.role.kind === 'main')
  const minors = cast.filter((member) => member.role.kind === 'minor')
  const toggle = (name: string) => {
    const next = new Set(expanded)
    if (!next.delete(name)) {
      next.add(name)
    }
    setExpanded(next)
  }

  return (
    <div className="flex flex-col gap-4">
      <Section title="旁白">
        {narrator ? (
          <NarratorStrip world={world} member={narrator} character={profile(narrator.character)} />
        ) : (
          <AddCast world={world} cast={cast} kind="narrator" look="strip" />
        )}
      </Section>
      <Section title="主要角色">
        <div className="flex flex-wrap items-start gap-3">
          {mains.map((member) => (
            <MainCastCard
              key={member.character}
              world={world}
              member={member}
              character={profile(member.character)}
              expanded={expanded.has(member.character)}
              onToggle={() => toggle(member.character)}
            />
          ))}
          <AddCast world={world} cast={cast} kind="main" look="card" />
        </div>
      </Section>
      <Section title="次要角色">
        <div className="flex flex-wrap items-center gap-2">
          {minors.map((member) => (
            <MinorCastChip key={member.character} world={world} member={member} />
          ))}
          <AddCast world={world} cast={cast} kind="minor" look="chip" />
        </div>
      </Section>
    </div>
  )
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div className="flex flex-col gap-1.5">
      <span className="text-xs text-muted-foreground">{title}</span>
      {children}
    </div>
  )
}
