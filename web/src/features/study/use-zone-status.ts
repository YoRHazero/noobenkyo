import { useQuery } from '@tanstack/react-query'
import { commands, type Zone } from '@/bindings'
import { unwrap } from '@/components/result'
import { studyKeys } from '@/features/study/study-keys'

export function useZoneStatus(zone: Zone) {
  return useQuery({
    queryKey: studyKeys.zoneStatus(zone),
    queryFn: () => unwrap(commands.zoneStatus(zone)),
  })
}
