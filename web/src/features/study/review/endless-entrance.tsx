import { useRef, useState } from 'react'
import { cn } from '@/components/ui/cn'
import styles from '@/features/study/review/endless-effect.module.css'
import { EndlessBackdrop } from '@/features/study/review/endless-backdrop'
import { InfinityMark } from '@/features/study/review/infinity-mark'
import { ReviewEntrance } from '@/features/study/review/review-entrance'

type Props = {
  /** As the home page counts it; `undefined` while it loads. */
  toReview: number | undefined
  disabled: boolean
  onClick: () => void
}

/** The endless entrance; hovering it turns the whole window into its effect. */
export function EndlessEntrance({ toReview, disabled, onClick }: Props) {
  const card = useRef<HTMLButtonElement>(null)
  const [active, setActive] = useState(false)
  const [origin, setOrigin] = useState({ x: 0, y: 0 })

  const enter = () => {
    const box = card.current?.getBoundingClientRect()
    if (box) {
      setOrigin({ x: box.left + box.width / 2, y: box.top + box.height / 2 })
    }
    setActive(true)
  }

  return (
    <>
      <EndlessBackdrop active={active} origin={origin} />
      <ReviewEntrance
        ref={card}
        title={<span className={styles.title}>无限脑力</span>}
        label="待复习"
        count={toReview}
        disabled={disabled}
        active={active}
        className={cn(styles.card, 'hover:shadow-sm')}
        onClick={onClick}
        onMouseEnter={enter}
        onMouseLeave={() => setActive(false)}
      >
        <span className={styles.ring} aria-hidden />
        <InfinityMark />
      </ReviewEntrance>
    </>
  )
}
