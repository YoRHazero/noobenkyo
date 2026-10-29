import { useMutation } from '@tanstack/react-query'
import { commands, type TaskId } from '@/bindings'
import { unwrap } from '@/backend/unwrap'

/** Stops a task; its end arrives as an event, like any other end. */
export function useAiCancel() {
  return useMutation({
    mutationFn: (task: TaskId) => unwrap(commands.aiCancel(task)),
  })
}
