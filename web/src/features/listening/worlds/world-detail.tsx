import { useListeningEpisodes } from '@/backend/listening/use-listening-episodes'
import { useListeningWorld } from '@/backend/listening/use-listening-world'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { ImportScriptButton } from '@/features/listening/script/import-script-button'
import { CastList } from '@/features/listening/worlds/cast/cast-list'
import { DeleteWorldButton } from '@/features/listening/worlds/delete-world-button'
import { EpisodeList } from '@/features/listening/worlds/episodes/episode-list'
import { WorldSection } from '@/features/listening/worlds/world-section'
import { WorldSetting } from '@/features/listening/worlds/world-setting'

type Props = {
  name: string
  onDeleted: () => void
}

/**
 * A world: its setting, its episodes (the most used, so first), and its
 * cast. Generating an episode is not there yet, so its button is reserved.
 */
export function WorldDetail({ name, onDeleted }: Props) {
  const world = useListeningWorld(name)
  const episodes = useListeningEpisodes(name)
  const error = world.error ?? episodes.error
  if (error) {
    return <ErrorText error={error} />
  }
  if (world.data === undefined || episodes.data === undefined) {
    return <p className="text-sm text-muted-foreground">加载中…</p>
  }
  if (world.data === null) {
    return <p className="text-sm text-muted-foreground">无</p>
  }
  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-1">
        <h2 className="text-xl font-semibold">{world.data.name}</h2>
        <WorldSetting world={world.data.name} setting={world.data.setting} />
      </div>
      <WorldSection
        title="集"
        actions={
          <>
            <ImportScriptButton
              target={{ kind: 'new', world: name }}
              label="导入剧本…"
              title={`导入剧本：${name}的新一集`}
            />
            <Button variant="outline" size="sm" disabled>
              生成…
            </Button>
          </>
        }
      >
        <EpisodeList world={name} episodes={episodes.data} />
      </WorldSection>
      <WorldSection title="可出场角色">
        <CastList world={name} cast={world.data.cast} />
      </WorldSection>
      <DeleteWorldButton world={name} episodes={episodes.data.length} onDeleted={onDeleted} />
    </div>
  )
}
