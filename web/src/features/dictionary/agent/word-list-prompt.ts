/** A new word list on a topic, written down to its sentences. */
export type WordListTask = {
  /** Blank leaves the topic to the agent. */
  topic: string
  /** How many words. */
  count: number
  /** How many sentences each sense gets. */
  perSense: number
}

/** The starting prompt for writing the list and sending it to the app. */
export function wordListPrompt(task: WordListTask): string {
  const topicText = task.topic.trim()
  const topic = topicText === '' ? '没有指定，自己选一个日常的话题' : topicText
  return `用 noobenkyo 命令行生成一份日语词表，送进 app。用法先运行 \`noobenkyo dictionary guide\` 看。\n\n话题：${topic}\n个数：${task.count}\n只列动词、形容词、副词和名词，写到例句，每个义项 ${task.perSense} 条例句。词库里已有的词不要。`
}
