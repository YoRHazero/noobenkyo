import type { ReactNode } from 'react'
import { useListeningCreateCharacter } from '@/backend/listening/use-listening-create-character'
import { ChoiceButton } from '@/components/choice-button'
import { ErrorText } from '@/components/error-text'
import { Button } from '@/components/ui/button'
import { DialogContent, DialogFooter, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { languageNames, languages } from '@/features/listening/language-names'
import { genderNames, genders } from '@/features/listening/characters/gender-names'
import { KanaField } from '@/features/listening/characters/kana-field'
import { PromptField } from '@/features/listening/characters/prompt-field'
import { ReferenceField } from '@/features/listening/characters/reference-field'
import { useCharacterDraft } from '@/features/listening/characters/use-character-draft'

type Props = {
  onCreated: (name: string) => void
}

/**
 * Creates a character, with the references uploaded or none: they can be
 * added later, and only synthesis needs them. A rejected form keeps what was
 * typed and lists every problem; while creating, it cannot be closed.
 */
export function NewCharacterDialog({ onCreated }: Props) {
  const draft = useCharacterDraft()
  const create = useListeningCreateCharacter()
  const busy = create.isPending
  const keepOpen = (event: Event) => {
    if (busy) {
      event.preventDefault()
    }
  }
  const submit = async () => {
    const input = draft.input()
    if (input === null) {
      return
    }
    const problems = await create.mutateAsync(input).catch(() => null)
    if (problems?.length === 0) {
      onCreated(input.name.trim())
    }
  }

  return (
    <DialogContent
      aria-describedby={undefined}
      showCloseButton={!busy}
      onEscapeKeyDown={keepOpen}
      onInteractOutside={keepOpen}
      className="sm:max-w-2xl"
    >
      <DialogTitle>新建角色</DialogTitle>
      <div className="grid grid-cols-[3.5rem_1fr] items-start gap-x-3 gap-y-2.5 text-sm">
        <FormRow label="名字">
          <div className="grid grid-cols-2 gap-4">
            <Input
              aria-label="名字"
              value={draft.name}
              disabled={busy}
              className="h-8"
              onChange={(event) => draft.setName(event.target.value)}
            />
            <InlineField label="读音">
              <KanaField
                value={draft.kana}
                disabled={busy || draft.kanaFollowsName}
                onChange={draft.setKana}
              />
            </InlineField>
          </div>
        </FormRow>
        <FormRow label="性别">
          <div className="grid grid-cols-2 gap-4">
            <div className="flex gap-1">
              {genders.map((gender) => (
                <ChoiceButton
                  key={gender}
                  chosen={draft.gender === gender}
                  disabled={busy}
                  onClick={() => draft.setGender(gender)}
                >
                  {genderNames[gender]}
                </ChoiceButton>
              ))}
            </div>
            <InlineField label="年龄">
              <Input
                aria-label="年龄"
                inputMode="numeric"
                value={draft.ageText}
                aria-invalid={!draft.ageValid}
                disabled={busy}
                className="h-8 w-16"
                onChange={(event) => draft.setAgeText(event.target.value)}
              />
              <span className="text-muted-foreground">岁</span>
            </InlineField>
          </div>
        </FormRow>
        <FormRow label="prompt">
          <PromptField value={draft.prompt} disabled={busy} onChange={draft.setPrompt} />
        </FormRow>
        <FormRow label="参考音">
          <div className="grid grid-cols-2 gap-4">
            {languages.map((language) => (
              <div key={language} className="flex min-w-0 flex-col gap-1">
                <span className="text-xs text-muted-foreground">{languageNames[language]}</span>
                <ReferenceField
                  language={language}
                  draft={draft.references[language]}
                  disabled={busy}
                  onChange={(reference) => draft.setReference(language, reference)}
                />
              </div>
            ))}
          </div>
        </FormRow>
      </div>
      {create.data && create.data.length > 0 && (
        <ul className="flex flex-col gap-1 text-sm text-destructive">
          {create.data.map((problem) => (
            <li key={problem}>{problem}</li>
          ))}
        </ul>
      )}
      <ErrorText error={create.error} />
      <DialogFooter>
        <Button disabled={busy || !draft.ageValid} onClick={submit}>
          新建
        </Button>
      </DialogFooter>
    </DialogContent>
  )
}

function FormRow({ label, children }: { label: string; children: ReactNode }) {
  return (
    <>
      <span className="pt-1.5 text-muted-foreground">{label}</span>
      <div className="min-w-0">{children}</div>
    </>
  )
}

/** A labelled field sharing a row with another one. */
function InlineField({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex min-w-0 items-center gap-2">
      <span className="shrink-0 text-muted-foreground">{label}</span>
      {children}
    </div>
  )
}
