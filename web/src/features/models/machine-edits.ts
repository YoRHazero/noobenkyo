import type { MachineDraft } from '@/features/models/models-draft'

/** A machine the user has just added: everything still to fill in. */
export function addMachine(machines: MachineDraft[]): MachineDraft[] {
  const key = Math.max(-1, ...machines.map((machine) => machine.key)) + 1
  const added: MachineDraft = {
    key,
    name: '',
    ssh: '',
    prefixes: [],
    models: '',
    qwenTts: '',
    llamaCpp: '',
    llamaPython: '',
    runtimes: [],
  }
  return [...machines, added]
}

export function removeMachine(machines: MachineDraft[], key: number): MachineDraft[] {
  return machines.filter((machine) => machine.key !== key)
}

/** Up (-1) or down (1) in the order of preference; at either end, nothing moves. */
export function moveMachine(machines: MachineDraft[], key: number, by: -1 | 1): MachineDraft[] {
  const from = machines.findIndex((machine) => machine.key === key)
  const to = from + by
  if (from === -1 || to < 0 || to >= machines.length) {
    return machines
  }
  const moved = [...machines]
  ;[moved[from], moved[to]] = [moved[to] as MachineDraft, moved[from] as MachineDraft]
  return moved
}

export function changeMachine(
  machines: MachineDraft[],
  key: number,
  change: Partial<Omit<MachineDraft, 'key'>>,
): MachineDraft[] {
  return machines.map((machine) => (machine.key === key ? { ...machine, ...change } : machine))
}
