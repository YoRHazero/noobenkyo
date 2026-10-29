import { GalleryVerticalEnd } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

/**
 * Draws a premise, as a card is drawn from a deck. Disabled for now: there
 * is nothing to draw from yet.
 */
export function DrawPremiseButton() {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        {/* A disabled button gets no pointer events, so the tooltip hangs on this. */}
        <span>
          <Button variant="ghost" size="icon-sm" aria-label="抽取" disabled>
            <GalleryVerticalEnd />
          </Button>
        </span>
      </TooltipTrigger>
      <TooltipContent>抽取</TooltipContent>
    </Tooltip>
  )
}
