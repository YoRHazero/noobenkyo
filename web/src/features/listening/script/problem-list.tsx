type Props = {
  problems: string[]
}

/** Why a script or an edit does not pass, one line each; nothing when it does. */
export function ProblemList({ problems }: Props) {
  if (problems.length === 0) {
    return null
  }
  return (
    <ul className="flex flex-col gap-1 text-sm text-destructive">
      {problems.map((problem) => (
        <li key={problem}>{problem}</li>
      ))}
    </ul>
  )
}
