import { Link, useNavigate } from '@tanstack/react-router'
import { useListeningWorlds } from '@/backend/listening/use-listening-worlds'
import { ErrorText } from '@/components/error-text'
import { ListeningPage } from '@/features/listening/listening-page'
import { MasterDetail } from '@/features/listening/master-detail'
import { masterItemClass } from '@/features/listening/master-item-class'
import { nextSelection } from '@/features/listening/next-selection'
import { NewWorldButton } from '@/features/listening/worlds/new-world-button'
import { WorldDetail } from '@/features/listening/worlds/world-detail'

type Props = {
  /** The world in the address; the first one when there is none. */
  selected?: string
}

/** Every world by name, and the chosen one with its episodes and cast. */
export function WorldsTab({ selected }: Props) {
  const worlds = useListeningWorlds()
  const navigate = useNavigate()
  const names = worlds.data?.map((world) => world.name) ?? []
  const chosen = selected ?? names[0] ?? null
  const show = (name: string | null) =>
    name === null
      ? navigate({ to: '/listening/worlds' })
      : navigate({ to: '/listening/worlds/$name', params: { name } })

  return (
    <ListeningPage tab="worlds" actions={<NewWorldButton onCreated={show} />}>
      {worlds.error ? (
        <ErrorText error={worlds.error} />
      ) : !worlds.data ? (
        <p className="text-sm text-muted-foreground">加载中…</p>
      ) : (
        <MasterDetail
          list={names.map((name) => (
            <Link
              key={name}
              to="/listening/worlds/$name"
              params={{ name }}
              className={masterItemClass(name === chosen)}
            >
              {name}
            </Link>
          ))}
          detail={
            chosen === null ? (
              <p className="text-sm text-muted-foreground">无世界</p>
            ) : (
              <WorldDetail
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
