import { describe, expect, it } from 'vitest'
import {
  addMachine,
  changeMachine,
  moveMachine,
  removeMachine,
} from '@/features/models/machine-edits'
import { yorha } from '@/features/models/test-machines'

const three = [0, 1, 2].map((key) => ({ ...yorha(), key, name: `m${key}` }))
const names = (machines: { name: string }[]) => machines.map((machine) => machine.name)

describe('editing the machines', () => {
  it('adds a blank one under a new key', () => {
    const added = addMachine(three)
    expect(added).toHaveLength(4)
    expect(added[3]).toMatchObject({ key: 3, name: '', prefixes: [], runtimes: [] })
    expect(addMachine([])[0]?.key).toBe(0)
  })

  it('moves one up or down, but not past either end', () => {
    expect(names(moveMachine(three, 1, -1))).toEqual(['m1', 'm0', 'm2'])
    expect(names(moveMachine(three, 1, 1))).toEqual(['m0', 'm2', 'm1'])
    expect(moveMachine(three, 0, -1)).toBe(three)
    expect(moveMachine(three, 2, 1)).toBe(three)
  })

  it('changes and removes one by its key', () => {
    expect(names(changeMachine(three, 2, { name: 'zero' }))).toEqual(['m0', 'm1', 'zero'])
    expect(names(removeMachine(three, 1))).toEqual(['m0', 'm2'])
  })
})
