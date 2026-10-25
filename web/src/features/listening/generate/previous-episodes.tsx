import { Plus, X } from 'lucide-react'
import { Reorder } from 'motion/react'
import type { EpisodeSummary } from '@/bindings'
import { episodeLabel } from '@/features/listening/episode-label'

type Props = {
  /** The episodes that may come before, in their order. */
  episodes: EpisodeSummary[]
  /** The chosen ones' uuids, in the order the agent is to read them. */
  chosen: string[]
  onChange: (chosen: string[]) => void
}

/**
 * The episodes before the one to write, as in the question order of the
 * settings: the chosen ones in a column to drag into order, × to take one
 * out; the others below, where a click adds one at the end. None chosen
 * leaves the choice to the agent.
 */
export function PreviousEpisodes({ episodes, chosen, onChange }: Props) {
  const byUuid = new Map(episodes.map((episode) => [episode.uuid, episode]))
  const others = episodes.filter((episode) => !chosen.includes(episode.uuid))
  return (
    <div className="flex flex-col gap-3">
      {chosen.length === 0 ? (
        <p className="text-sm text-muted-foreground">无：交给 agent 判断</p>
      ) : (
        <Reorder.Group
          axis="y"
          values={chosen}
          onReorder={onChange}
          className="flex flex-col gap-1.5"
        >
          {chosen.map((uuid, index) => {
            const episode = byUuid.get(uuid)
            return (
              episode && (
                <Reorder.Item
                  key={uuid}
                  value={uuid}
                  whileDrag={{ scale: 1.02, boxShadow: '0 6px 16px rgb(0 0 0 / 0.15)' }}
                  className="flex cursor-grab items-center gap-2 rounded-md border bg-card py-1 pr-1 pl-2.5 text-sm shadow-xs select-none active:cursor-grabbing"
                >
                  <span className="w-4 text-xs text-muted-foreground tabular-nums">
                    {index + 1}
                  </span>
                  <EpisodeName episode={episode} />
                  <button
                    type="button"
                    aria-label={`去掉${episode.title}`}
                    className="ml-auto rounded-sm p-0.5 text-muted-foreground hover:bg-accent hover:text-foreground"
                    // A press on × is not the start of a drag.
                    onPointerDown={(event) => event.stopPropagation()}
                    onClick={() => onChange(chosen.filter((u) => u !== uuid))}
                  >
                    <X className="size-3.5" />
                  </button>
                </Reorder.Item>
              )
            )
          })}
        </Reorder.Group>
      )}
      {others.length > 0 && (
        <div className="flex flex-col gap-1">
          <p className="text-xs text-muted-foreground">其他集</p>
          <div className="flex max-h-48 flex-col gap-1 overflow-y-auto">
            {others.map((episode) => (
              <button
                key={episode.uuid}
                type="button"
                className="flex items-center gap-2 rounded-md border border-dashed px-2.5 py-1 text-left text-sm text-muted-foreground hover:bg-accent hover:text-foreground"
                onClick={() => onChange([...chosen, episode.uuid])}
              >
                <Plus className="size-3.5 shrink-0" />
                <EpisodeName episode={episode} />
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}

function EpisodeName({ episode }: { episode: EpisodeSummary }) {
  return (
    <span className="min-w-0 truncate">
      <span className="mr-2 text-muted-foreground tabular-nums">
        {episodeLabel(episode.number)}
      </span>
      {episode.title}
    </span>
  )
}
