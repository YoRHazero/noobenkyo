import {
  createRootRoute,
  createRoute,
  createRouter,
  Outlet,
  redirect,
} from '@tanstack/react-router'
import { HomePage } from '@/app/home-page'
import { Layout } from '@/app/layout'
import { CharacterRoute, EpisodeRoute, TheaterRoute, WorldRoute } from '@/app/listening-routes'
import { DictionaryPage } from '@/features/dictionary/dictionary-page'
import { CharactersTab } from '@/features/listening/characters/characters-tab'
import { WorldsTab } from '@/features/listening/worlds/worlds-tab'
import { SettingsPage } from '@/features/settings/settings-page'
import { LearnPage } from '@/features/study/learn/learn-page'
import { ReviewPage } from '@/features/study/review/review-page'

const rootRoute = createRootRoute({ component: Outlet })

/** Every page but the theater: under the top navigation. */
const appRoute = createRoute({ getParentRoute: () => rootRoute, id: 'app', component: Layout })

const worldRoute = createRoute({
  getParentRoute: () => appRoute,
  path: '/listening/worlds/$name',
  component: WorldRoute,
})

const characterRoute = createRoute({
  getParentRoute: () => appRoute,
  path: '/listening/characters/$name',
  component: CharacterRoute,
})

const episodeRoute = createRoute({
  getParentRoute: () => appRoute,
  path: '/listening/episodes/$uuid',
  component: EpisodeRoute,
})

/** The theater takes the whole window, so it is outside the navigation. */
const theaterRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/listening/worlds/$name/play',
  validateSearch: (search: Record<string, unknown>): { from?: string } =>
    typeof search.from === 'string' ? { from: search.from } : {},
  component: TheaterRoute,
})

const routeTree = rootRoute.addChildren([
  appRoute.addChildren([
    createRoute({ getParentRoute: () => appRoute, path: '/', component: HomePage }),
    createRoute({ getParentRoute: () => appRoute, path: '/learn', component: LearnPage }),
    createRoute({ getParentRoute: () => appRoute, path: '/review', component: ReviewPage }),
    createRoute({ getParentRoute: () => appRoute, path: '/dictionary', component: DictionaryPage }),
    createRoute({ getParentRoute: () => appRoute, path: '/settings', component: SettingsPage }),
    createRoute({
      getParentRoute: () => appRoute,
      path: '/listening',
      beforeLoad: () => {
        throw redirect({ to: '/listening/worlds' })
      },
    }),
    createRoute({
      getParentRoute: () => appRoute,
      path: '/listening/worlds',
      component: () => <WorldsTab />,
    }),
    worldRoute,
    createRoute({
      getParentRoute: () => appRoute,
      path: '/listening/characters',
      component: () => <CharactersTab />,
    }),
    characterRoute,
    episodeRoute,
  ]),
  theaterRoute,
])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
