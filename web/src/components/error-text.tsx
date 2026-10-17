type Props = {
  error: Error | null
}

/** Shows an error message, or nothing. */
export function ErrorText({ error }: Props) {
  if (!error) {
    return null
  }
  return <p className="text-sm text-destructive">{error.message}</p>
}
