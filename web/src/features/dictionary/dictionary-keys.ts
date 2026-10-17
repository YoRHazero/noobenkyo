/** Query keys of the dictionary pages. */
export const dictionaryKeys = {
  all: ['dictionary'] as const,
  stats: ['dictionary', 'stats'] as const,
  senses: (query: string, page: number) => ['dictionary', 'senses', query, page] as const,
  logCount: ['dictionary', 'log-count'] as const,
}
