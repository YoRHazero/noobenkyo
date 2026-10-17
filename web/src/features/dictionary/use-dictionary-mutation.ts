import { useMutation, useQueryClient } from '@tanstack/react-query'
import { dictionaryKeys } from '@/features/dictionary/dictionary-keys'

/** A mutation that changes the dictionary; every dictionary query refreshes afterwards. */
export function useDictionaryMutation<T, R>(action: (value: T) => Promise<R>) {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: action,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: dictionaryKeys.all }),
  })
}
