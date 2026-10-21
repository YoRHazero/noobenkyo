type Props = {
  error: Error | null
}

/** A failed command's message as the backend wrote it, or nothing. */
export function ErrorText({ error }: Props) {
  if (!error) {
    return null
  }
  return <p className="text-sm text-destructive">{error.message}</p>
}
