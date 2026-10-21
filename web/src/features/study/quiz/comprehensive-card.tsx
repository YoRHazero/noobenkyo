import type { Card } from '@/bindings'
import { Covered } from '@/features/study/quiz/covered'
import { GlossList } from '@/features/study/quiz/gloss-list'

type Props = {
  card: Card
}

/** The face of a comprehensive question; the settings decide what starts covered. */
export function ComprehensiveCard({ card }: Props) {
  return (
    <div className="flex flex-col items-start gap-3">
      <Covered shown={card.visible.writing}>
        <span lang="ja" className="text-4xl">
          {card.writing}
        </span>
      </Covered>
      <Covered shown={card.visible.kana}>
        <span lang="ja" className="text-xl text-muted-foreground">
          {card.kana}
        </span>
      </Covered>
      <Covered shown={card.visible.glosses}>
        <GlossList glosses={card.glosses} />
      </Covered>
    </div>
  )
}
