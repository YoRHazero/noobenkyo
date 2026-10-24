import type { Role } from '@/bindings'

export type RoleKind = Role['kind']

/** The order roles are offered and listed in, the narrator first. */
export const roleKinds: readonly RoleKind[] = ['narrator', 'main', 'minor']

export const roleNames: Record<RoleKind, string> = {
  narrator: '旁白',
  main: '主要角色',
  minor: '次要角色',
}

/** The role a form describes; only a main character has a description. */
export function buildRole(kind: RoleKind, description: string): Role {
  return kind === 'main' ? { kind, description } : { kind }
}

/** Whether saving `to` over `from` deletes a main character's description. */
export function dropsDescription(from: Role | null, to: RoleKind): boolean {
  return from?.kind === 'main' && to !== 'main'
}
