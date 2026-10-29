import { createRootRoute, createRoute, createRouter, redirect } from '@tanstack/react-router'
import { endReview } from '@/backend/review/end-review'
import { FocusLayout } from '@/app/focus-layout'
import { HomePage } from '@/app/home-page'
import { Layout } from '@/app/layout'
import { Root } from '@/app/root'
import { CharacterRoute, EpisodeRoute, TheaterRoute, WorldRoute } from '@/app/listening-routes'
import { ReviewSessionRoute } from '@/app/study-routes'
import { DictionaryPage } from '@/features/dictionary/dictionary-page'
import { CharactersTab } from '@/features/listening/characters/characters-tab'
import { WorldsTab } from '@/features/listening/worlds/worlds-tab'
import { SettingsPage } from '@/features/settings/settings-page'
import { LearnPage } from '@/features/study/learn/learn-page'
import { ReviewPage } from '@/features/study/review/review-page'
import type { ReviewMode } from '@/features/study/review/review-mode'

const rootRoute = createRootRoute({ component: Root })

/** The pages under the top navigation. */
const appRoute = createRoute({ getParentRoute: () => rootRoute, id: 'app', component: Layout })

/** Learning and a review session: the whole window, without navigation. */
const focusRoute = createRoute({
  getParentRoute: () => rootRoute,
  id: 'focus',
  component: FocusLayout,
})

/**
 * Leaving the session's page ends the session. The router calls `onLeave`
 * only when the address is really left, never on a development remount.
 */
const reviewSessionRoute = createRoute({
  getParentRoute: () => focusRoute,
  path: '/review/session',
  validateSearch: (search: Record<string, unknown>): { mode: ReviewMode } => ({
    mode: search.mode === 'endless' ? 'endless' : 'group',
  }),
  onLeave: () => {
    endReview().catch(() => undefined)
  },
  component: ReviewSessionRoute,
})

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
  focusRoute.addChildren([
    createRoute({ getParentRoute: () => focusRoute, path: '/learn', component: LearnPage }),
    reviewSessionRoute,
  ]),
  theaterRoute,
])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
