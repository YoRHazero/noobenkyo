import { useState } from 'react'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import { Reorder } from 'motion/react'
import type { EpisodeSummary } from '@/bindings'
import { Button } from '@/components/ui/button'
import { EpisodeBox } from '@/features/listening/generate/episode-box'
import { EpisodeOption } from '@/features/listening/generate/episode-option'
import { ShownEpisode } from '@/features/listening/generate/shown-episode'
import {
  addPicked,
  removePicked,
  togglePicked,
  withoutPicks,
} from '@/features/listening/generate/transfer'

type Props = {
  /** The episodes that may come before, in their order. */
  episodes: EpisodeSummary[]
  /** The chosen ones' uuids, in the order the agent is to read them. */
  chosen: string[]
  onChange: (chosen: string[]) => void
}

const listClass = 'flex flex-1 flex-col gap-0.5 overflow-y-auto p-1'

/**
 * The episodes before the one to write, as a transfer list: the world's
 * episodes on the left in their order, the chosen ones on the right in the
 * order the agent reads them, dragged into order there. None chosen leaves
 * the choice to the agent. Under them, the episode last clicked.
 */
export function PreviousEpisodes({ episodes, chosen, onChange }: Props) {
  const [picked, setPicked] = useState<ReadonlySet<string>>(new Set())
  const [shown, setShown] = useState<string | null>(null)
  const byUuid = new Map(episodes.map((episode) => [episode.uuid, episode]))
  const others = episodes.filter((episode) => !chosen.includes(episode.uuid))
  const pickedOthers = others.filter((episode) => picked.has(episode.uuid))
  const pickedChosen = chosen.filter((uuid) => picked.has(uuid))

  const add = (uuids: string[]) => {
    onChange(
      addPicked(
        episodes.map((episode) => episode.uuid),
        chosen,
        new Set(uuids),
      ),
    )
    setPicked(withoutPicks(picked, uuids))
  }
  const remove = (uuids: string[]) => {
    onChange(removePicked(chosen, new Set(uuids)))
    setPicked(withoutPicks(picked, uuids))
  }
  const toggle = (uuid: string) => {
    setPicked(togglePicked(picked, uuid))
    setShown(uuid)
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="grid grid-cols-[1fr_auto_1fr] gap-3">
        <EpisodeBox title="已有剧本" empty="无" isEmpty={others.length === 0}>
          <ul role="listbox" aria-label="已有剧本" aria-multiselectable className={listClass}>
            {others.map((episode) => (
              <EpisodeOption
                key={episode.uuid}
                episode={episode}
                picked={picked.has(episode.uuid)}
                reorderable={false}
                onToggle={() => toggle(episode.uuid)}
                onMove={() => {
                  add([episode.uuid])
                  setShown(episode.uuid)
                }}
              />
            ))}
          </ul>
        </EpisodeBox>
        <div className="flex flex-col justify-center gap-2">
          <Button
            variant="outline"
            size="icon-sm"
            aria-label="加进前序剧本"
            disabled={pickedOthers.length === 0}
            onClick={() => add(pickedOthers.map((episode) => episode.uuid))}
          >
            <ChevronRight />
          </Button>
          <Button
            variant="outline"
            size="icon-sm"
            aria-label="移出前序剧本"
            disabled={pickedChosen.length === 0}
            onClick={() => remove(pickedChosen)}
          >
            <ChevronLeft />
          </Button>
        </div>
        <EpisodeBox title="前序剧本" empty="不指定" isEmpty={chosen.length === 0}>
          <Reorder.Group
            axis="y"
            values={chosen}
            onReorder={onChange}
            layoutScroll
            role="listbox"
            aria-label="前序剧本"
            aria-multiselectable
            className={listClass}
          >
            {chosen.map((uuid) => {
              const episode = byUuid.get(uuid)
              return (
                episode && (
                  <EpisodeOption
                    key={uuid}
                    episode={episode}
                    picked={picked.has(uuid)}
                    reorderable
                    onToggle={() => toggle(uuid)}
                    onMove={() => {
                      remove([uuid])
                      setShown(uuid)
                    }}
                  />
                )
              )
            })}
          </Reorder.Group>
        </EpisodeBox>
      </div>
      <ShownEpisode episode={shown === null ? undefined : byUuid.get(shown)} />
    </div>
  )
}
