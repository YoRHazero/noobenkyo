type Props = {
  glosses: string[]
}

/** Glosses, the primary one first. */
export function GlossList({ glosses }: Props) {
  const [primary, ...secondary] = glosses
  return (
    <p className="text-lg">
      <span className="font-medium">{primary}</span>
      {secondary.length > 0 && (
        <span className="text-muted-foreground">；{secondary.join('；')}</span>
      )}
    </p>
  )
}
