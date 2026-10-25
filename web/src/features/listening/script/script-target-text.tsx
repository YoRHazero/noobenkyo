import type { ScriptTarget } from '@/bindings'
import { useListeningEpisode } from '@/backend/listening/use-listening-episode'
import { episodeLabel } from '@/features/listening/episode-label'

type Props = {
  target: ScriptTarget
}

/** Where a script goes: 「星见町的新一集」, 「续写 第 2 集 雨の日」. */
export function ScriptTargetText({ target }: Props) {
  if (target.kind === 'newEpisode') {
    return <>{target.world}的新一集</>
  }
  return <AppendingText episode={target.episode} />
}

function AppendingText({ episode }: { episode: string }) {
  const found = useListeningEpisode(episode)
  if (found.data === null) {
    return <>续写 已删除的集</>
  }
  if (!found.data) {
    return <>续写 …</>
  }
  return (
    <>
      续写 {episodeLabel(found.data.number)} {found.data.title}
    </>
  )
}
