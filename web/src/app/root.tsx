import { Outlet } from '@tanstack/react-router'
import { useAiTaskEvents } from '@/backend/ai/use-ai-task-events'
import { nooChan } from '@/stores/no-chan'

/**
 * Above every page, the theater and the sessions included, so a task's
 * events are followed wherever the user is. A task that fails is something
 * the user started that went wrong, so no-chan reacts to it.
 */
export function Root() {
  useAiTaskEvents(nooChan.notifyError)
  return <Outlet />
}
