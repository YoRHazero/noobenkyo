import type { Language } from '@/bindings'
import { cn } from '@/components/ui/cn'

const short: Record<Language, string> = { ja: '日', zh: '中' }

/** The languages a cast member has references in: 「日 中」. */
export function CastLanguages({
  languages,
  className,
}: {
  languages: Language[]
  className?: string
}) {
  return (
    <span className={cn('text-xs tracking-wider', className)}>
      {languages.map((language) => short[language]).join(' ')}
    </span>
  )
}
