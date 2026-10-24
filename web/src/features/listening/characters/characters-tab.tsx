import { Link, useNavigate } from '@tanstack/react-router'
import { useListeningCharacters } from '@/backend/listening/use-listening-characters'
import { ErrorText } from '@/components/error-text'
import { ListeningPage } from '@/features/listening/listening-page'
import { MasterDetail } from '@/features/listening/master-detail'
import { masterItemClass } from '@/features/listening/master-item-class'
import { nextSelection } from '@/features/listening/next-selection'
import { CharacterDetail } from '@/features/listening/characters/character-detail'
import { NewCharacterButton } from '@/features/listening/characters/new-character-button'

type Props = {
  /** The character in the address; the first one when there is none. */
  selected?: string
}

/** Every character by name, and the chosen one in full. */
export function CharactersTab({ selected }: Props) {
  const characters = useListeningCharacters()
  const navigate = useNavigate()
  const names = characters.data?.map((character) => character.name) ?? []
  const chosen = selected ?? names[0] ?? null
  const show = (name: string | null) =>
    name === null
      ? navigate({ to: '/listening/characters' })
      : navigate({ to: '/listening/characters/$name', params: { name } })

  return (
    <ListeningPage tab="characters" actions={<NewCharacterButton onCreated={show} />}>
      {characters.error ? (
        <ErrorText error={characters.error} />
      ) : !characters.data ? (
        <p className="text-sm text-muted-foreground">加载中…</p>
      ) : (
        <MasterDetail
          list={names.map((name) => (
            <Link
              key={name}
              to="/listening/characters/$name"
              params={{ name }}
              className={masterItemClass(name === chosen)}
            >
              {name}
            </Link>
          ))}
          detail={
            chosen === null ? (
              <p className="text-sm text-muted-foreground">无角色</p>
            ) : (
              <CharacterDetail
                key={chosen}
                name={chosen}
                onDeleted={() => show(nextSelection(names, chosen))}
              />
            )
          }
        />
      )}
    </ListeningPage>
  )
}
