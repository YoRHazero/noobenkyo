import { commands, type ImportOptions } from '@/bindings'
import { unwrap } from '@/backend/unwrap'
import { useChange } from '@/backend/use-change'

type Input = {
  text: string
  options: ImportOptions
}

export function useDictionaryImport() {
  return useChange(({ text, options }: Input) => unwrap(commands.dictionaryImport(text, options)))
}
