import { getRouteApi } from '@tanstack/react-router'
import { CharactersTab } from '@/features/listening/characters/characters-tab'
import { EpisodePage } from '@/features/listening/episodes/episode-page'
import { TheaterPage } from '@/features/listening/theater/theater-page'
import { WorldsTab } from '@/features/listening/worlds/worlds-tab'

/** The listening pages whose address names what they show, given it from the address. */

const world = getRouteApi('/app/listening/worlds/$name')
const character = getRouteApi('/app/listening/characters/$name')
const episode = getRouteApi('/app/listening/episodes/$uuid')
const theater = getRouteApi('/listening/worlds/$name/play')

export function WorldRoute() {
  return <WorldsTab selected={world.useParams().name} />
}

export function CharacterRoute() {
  return <CharactersTab selected={character.useParams().name} />
}

export function EpisodeRoute() {
  return <EpisodePage uuid={episode.useParams().uuid} />
}

export function TheaterRoute() {
  return <TheaterPage world={theater.useParams().name} from={theater.useSearch().from} />
}
