import styles from '@/features/study/review/endless-effect.module.css'

/** A lemniscate, drawn with a normalized length so the trace can run along it. */
const path = 'M32 24 C38 14 52 14 52 24 C52 34 38 34 32 24 C26 14 12 14 12 24 C12 34 26 34 32 24 Z'

/** The infinity sign of the endless entrance; its trace runs while the card is active. */
export function InfinityMark() {
  return (
    <svg viewBox="0 0 64 48" className={styles.infinity} aria-hidden>
      <defs>
        <linearGradient id="endless-trace-gradient" x1="0" x2="1">
          <stop offset="0%" stopColor="#22d3ee" />
          <stop offset="50%" stopColor="#a5b4fc" />
          <stop offset="100%" stopColor="#f0abfc" />
        </linearGradient>
      </defs>
      <path d={path} pathLength={100} className={styles.infinityBase} />
      <path d={path} pathLength={100} className={styles.infinityTrace} />
    </svg>
  )
}
