import type { ReactNode } from 'react'
import { useCanGoBack, useNavigate, useRouter } from '@tanstack/react-router'
import { useListeningEpisodes } from '@/backend/listening/use-listening-episodes'
import { useListeningWorld } from '@/backend/listening/use-listening-world'
import { Theater } from '@/features/listening/theater/theater'

type Props = {
  world: string
  /** The episode to start from; the world's first when absent. */
  from?: string
}

/**
 * The theater for one world, over the whole window. ✕ goes back to the page
 * it was opened from.
 */
export function TheaterPage({ world, from }: Props) {
  const worldQuery = useListeningWorld(world)
  const episodes = useListeningEpisodes(world)
  const router = useRouter()
  const canGoBack = useCanGoBack()
  const navigate = useNavigate()
  const close = () =>
    canGoBack
      ? router.history.back()
      : navigate({ to: '/listening/worlds/$name', params: { name: world } })

  const error = worldQuery.error ?? episodes.error
  if (error) {
    return <Frame>{error.message}</Frame>
  }
  if (worldQuery.data === undefined || episodes.data === undefined) {
    return <Frame>加载中…</Frame>
  }
  if (worldQuery.data === null || episodes.data.length === 0) {
    return <Frame>无集</Frame>
  }
  return (
    <Theater
      world={world}
      cast={worldQuery.data.cast}
      episodes={episodes.data}
      from={from}
      onClose={close}
    />
  )
}

function Frame({ children }: { children: ReactNode }) {
  return (
    <div className="fixed inset-0 flex items-center justify-center bg-black text-sm text-white/50">
      {children}
    </div>
  )
}
