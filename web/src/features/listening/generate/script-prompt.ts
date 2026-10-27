/**
 * A script to write, as chosen in the generation form: a new episode of
 * the world, or more of one of its episodes.
 */
export type ScriptTask = {
  world: string
  /** The uuid of the episode to continue; `null` for a new episode. */
  appending: string | null
  /** The episodes that come before, in the order of the story. Empty leaves the choice to the agent. */
  previous: string[]
  /** What happens in it. Blank leaves it to the agent. */
  premise: string
}

/**
 * The starting prompt the form copies for an agent: the task, the guide to
 * run first, and what the user chose. How to go about it is the guide's;
 * what the user left open is said to be the agent's to judge.
 */
export function scriptPrompt(task: ScriptTask): string {
  const goal =
    task.appending === null
      ? `给听力的世界「${task.world}」写新的一集剧本`
      : `续写听力的世界「${task.world}」里的一集（${task.appending}）`
  const premiseText = task.premise.trim()
  const premise =
    premiseText === ''
      ? '引子：没有指定，自己按世界设定和之前的剧情想一个。'
      : `引子：\n${premiseText}`
  const previous =
    task.previous.length === 0
      ? '前序集：没有指定，自己判断要参考这个世界里的哪些集。'
      : `前序集，按这个顺序：\n${task.previous.join('\n')}`
  return `用 noobenkyo 命令行${goal}，送进 app。用法先运行 \`noobenkyo listening guide\` 看。\n\n${premise}\n\n${previous}`
}
