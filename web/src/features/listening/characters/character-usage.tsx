import { Link } from '@tanstack/react-router'
import { useListeningCharacterUsage } from '@/backend/listening/use-listening-character-usage'
import { ErrorText } from '@/components/error-text'

/** The worlds the character is cast in, each a way into that world. */
export function CharacterUsage({ name }: { name: string }) {
  const usage = useListeningCharacterUsage(name)
  if (usage.error) {
    return <ErrorText error={usage.error} />
  }
  if (!usage.data) {
    return <span className="text-muted-foreground">加载中…</span>
  }
  if (usage.data.length === 0) {
    return <span className="text-muted-foreground">无</span>
  }
  return (
    <span className="flex flex-wrap gap-x-3">
      {usage.data.map((world) => (
        <Link
          key={world}
          to="/listening/worlds/$name"
          params={{ name: world }}
          className="underline-offset-4 hover:underline"
        >
          {world}
        </Link>
      ))}
    </span>
  )
}
