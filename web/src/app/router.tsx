import { createRootRoute, createRoute, createRouter } from '@tanstack/react-router'
import { HomePage } from '@/app/home-page'
import { Layout } from '@/app/layout'
import { DictionaryPage } from '@/features/dictionary/dictionary-page'
import { SettingsPage } from '@/features/settings/settings-page'
import { LearnPage } from '@/features/study/learn/learn-page'
import { ReviewPage } from '@/features/study/review/review-page'

const rootRoute = createRootRoute({ component: Layout })

const routeTree = rootRoute.addChildren([
  createRoute({ getParentRoute: () => rootRoute, path: '/', component: HomePage }),
  createRoute({ getParentRoute: () => rootRoute, path: '/learn', component: LearnPage }),
  createRoute({ getParentRoute: () => rootRoute, path: '/review', component: ReviewPage }),
  createRoute({ getParentRoute: () => rootRoute, path: '/dictionary', component: DictionaryPage }),
  createRoute({ getParentRoute: () => rootRoute, path: '/settings', component: SettingsPage }),
])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
