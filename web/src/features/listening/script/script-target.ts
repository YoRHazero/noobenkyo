/** Where a script goes: a new episode of a world, or the end of an episode. */
export type ScriptTarget = { kind: 'new'; world: string } | { kind: 'append'; episode: string }
