import { Button } from '@/components/ui/button'

type Props = {
  /** 0-based. */
  page: number
  pages: number
  onPage: (page: number) => void
}

/** "1 / 3" with the previous and next page; nothing for a single page. */
export function Pager({ page, pages, onPage }: Props) {
  if (pages <= 1) {
    return null
  }
  return (
    <div className="flex items-center justify-end gap-3 text-sm">
      <span className="text-muted-foreground tabular-nums">
        {page + 1} / {pages}
      </span>
      <Button variant="outline" size="sm" disabled={page === 0} onClick={() => onPage(page - 1)}>
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={page + 1 >= pages}
        onClick={() => onPage(page + 1)}
      >
        下一页
      </Button>
    </div>
  )
}
