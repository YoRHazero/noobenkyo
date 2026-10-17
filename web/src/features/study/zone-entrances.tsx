import { Link } from '@tanstack/react-router'
import type { Zone } from '@/bindings'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { zoneLabel } from '@/features/study/labels'
import { useZoneStatus } from '@/features/study/use-zone-status'
import { ZoneSummary } from '@/features/study/zone-summary'

/** The two entrances: learning and review. */
export function ZoneEntrances() {
  return (
    <div className="grid grid-cols-2 gap-4">
      <ZoneEntrance zone="Learning" to="/learn" />
      <ZoneEntrance zone="Review" to="/review" />
    </div>
  )
}

function ZoneEntrance({ zone, to }: { zone: Zone; to: '/learn' | '/review' }) {
  const status = useZoneStatus(zone)
  return (
    <Link to={to} className="rounded-xl transition-shadow hover:shadow-md">
      <Card className="h-full">
        <CardHeader>
          <CardTitle className="text-xl">{zoneLabel[zone]}</CardTitle>
        </CardHeader>
        <CardContent>{status.data && <ZoneSummary status={status.data} />}</CardContent>
      </Card>
    </Link>
  )
}
