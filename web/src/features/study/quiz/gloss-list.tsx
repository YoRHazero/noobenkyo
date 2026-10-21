type Props = {
  glosses: string[]
}

/** Glosses on one line, the primary one first and stronger. */
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
