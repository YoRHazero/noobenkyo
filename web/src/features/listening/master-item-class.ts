import { cn } from '@/components/ui/cn'

/** A name in the left list of `MasterDetail`, marked while chosen. */
export function masterItemClass(chosen: boolean): string {
  return cn(
    'truncate rounded-md px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground',
    chosen && 'bg-accent font-medium text-foreground',
  )
}
