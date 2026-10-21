import { Search } from 'lucide-react'
import { Input } from '@/components/ui/input'
import { CompletenessFilter } from '@/features/dictionary/browse/completeness-filter'
import { ProgressFilter } from '@/features/dictionary/browse/progress-filter'
import type { BrowseFilter } from '@/features/dictionary/browse/use-browse-filter'

type Props = {
  browse: BrowseFilter
}

/** The keyword, the layers and the progress the page and the export use. */
export function BrowseSearchBar({ browse }: Props) {
  return (
    <div className="flex flex-wrap items-center gap-x-5 gap-y-2">
      <div className="relative">
        <Search className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={browse.text}
          placeholder="写法、读音、释义"
          aria-label="搜索"
          className="h-8 w-64 pl-8"
          onChange={(event) => browse.setText(event.target.value)}
        />
      </div>
      <CompletenessFilter
        value={browse.completeness}
        progressChosen={browse.progress !== 'Any'}
        onChange={browse.setCompleteness}
      />
      <ProgressFilter value={browse.progress} onChange={browse.setProgress} />
    </div>
  )
}
