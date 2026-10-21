import { createPortal } from 'react-dom'
import styles from '@/features/study/review/endless-effect.module.css'
import { WarpField } from '@/features/study/review/warp-field'

type Props = {
  active: boolean
  /** The card's center, where the stars stream out from. */
  origin: { x: number; y: number }
}

/**
 * The whole window behind the endless entrance while it is hovered: dark,
 * blurred, an aurora drifting and stars streaming out of the card. It stays
 * mounted so it can fade in and out.
 */
export function EndlessBackdrop({ active, origin }: Props) {
  return createPortal(
    <div className={styles.backdrop} data-active={active} aria-hidden>
      <div className={styles.aurora} />
      <WarpField active={active} origin={origin} />
    </div>,
    document.body,
  )
}
