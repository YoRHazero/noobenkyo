import type { PreviewLine } from '@/bindings'

/** One line as a script writes it: the Japanese with its reading marks. */
export type LineDraft = {
  speaker: string
  ja: string
  zh: string
}

export type HeaderDraft = {
  title: string
  summary: string
}

/** A passing script taken apart, to be written out again with one edit. */
export type ScriptParts = HeaderDraft & {
  lines: LineDraft[]
}

/** The parts of a passing preview, the Japanese as written rather than split for ruby. */
export function scriptParts(preview: {
  title: string
  summary: string
  lines: PreviewLine[]
}): ScriptParts {
  return {
    title: preview.title,
    summary: preview.summary,
    lines: preview.lines.map(({ speaker, ja, zh }) => ({ speaker, ja, zh })),
  }
}

/**
 * The script in the format every source delivers, indented by two spaces.
 * Fields are written as given: trimming is the check's business.
 */
export function scriptText({ title, summary, lines }: ScriptParts): string {
  return JSON.stringify(
    { title, summary, lines: lines.map(({ speaker, ja, zh }) => ({ speaker, ja, zh })) },
    null,
    2,
  )
}

/** The script with its title and summary replaced. */
export function replaceHeader(parts: ScriptParts, header: HeaderDraft): string {
  return scriptText({ ...parts, ...header })
}

/** The script with the line at `index` (from 0) replaced. */
export function replaceLine(parts: ScriptParts, index: number, line: LineDraft): string {
  return scriptText({ ...parts, lines: parts.lines.map((old, at) => (at === index ? line : old)) })
}

/** The script without the line at `index` (from 0). */
export function deleteLine(parts: ScriptParts, index: number): string {
  return scriptText({ ...parts, lines: parts.lines.filter((_, at) => at !== index) })
}

/** The script with a line put before the one at `at` (from 0); at the length, it goes last. */
export function insertLine(parts: ScriptParts, at: number, line: LineDraft): string {
  return scriptText({
    ...parts,
    lines: [...parts.lines.slice(0, at), line, ...parts.lines.slice(at)],
  })
}
