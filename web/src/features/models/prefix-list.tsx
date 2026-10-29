import { Plus, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

type Props = {
  /** Outermost first; blank ones are dropped on saving. */
  prefixes: string[]
  onChange: (prefixes: string[]) => void
}

/** The commands wrapped around everything run on the machine, such as `wsl -e`. */
export function PrefixList({ prefixes, onChange }: Props) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      {prefixes.map((prefix, index) => (
        <div key={index} className="flex items-center">
          <Input
            aria-label={`第 ${index + 1} 层前缀`}
            value={prefix}
            className="h-8 w-32"
            onChange={(event) =>
              onChange(prefixes.map((p, i) => (i === index ? event.target.value : p)))
            }
          />
          <Button
            variant="ghost"
            size="icon-xs"
            aria-label="删除前缀"
            onClick={() => onChange(prefixes.filter((_, i) => i !== index))}
          >
            <X />
          </Button>
        </div>
      ))}
      <Button variant="ghost" size="sm" onClick={() => onChange([...prefixes, ''])}>
        <Plus />
        前缀
      </Button>
    </div>
  )
}
