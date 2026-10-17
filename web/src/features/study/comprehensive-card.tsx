import type { Card } from '@/bindings'
import { Covered } from '@/features/study/covered'
import { GlossList } from '@/features/study/gloss-list'

type Props = {
  card: Card
  revealed: boolean
}

/** The face of a comprehensive question; hidden parts are covered until revealed. */
export function ComprehensiveCard({ card, revealed }: Props) {
  return (
    <div className="flex flex-col items-start gap-3">
      <Covered shown={revealed || card.visible.writing}>
        <span lang="ja" className="text-4xl">
          {card.writing}
        </span>
      </Covered>
      <Covered shown={revealed || card.visible.kana}>
        <span lang="ja" className="text-xl text-muted-foreground">
          {card.kana}
        </span>
      </Covered>
      <Covered shown={revealed || card.visible.glosses}>
        <GlossList glosses={card.glosses} />
      </Covered>
    </div>
  )
}
