import { useState } from 'react'
import { useListeningCharacter } from '@/backend/listening/use-listening-character'
import { DetailItem, DetailList } from '@/components/detail-list'
import { ErrorText } from '@/components/error-text'
import { languageNames, languages } from '@/features/listening/language-names'
import { CharacterUsage } from '@/features/listening/characters/character-usage'
import { DeleteCharacterButton } from '@/features/listening/characters/delete-character-button'
import { genderNames } from '@/features/listening/characters/gender-names'
import { ReferenceRow } from '@/features/listening/characters/reference-row'
import { randomSeed } from '@/features/listening/characters/voice-design'

type Props = {
  name: string
  onDeleted: () => void
}

/**
 * A character as it was created, plus the missing references it can still
 * get. Both missing ones start from one seed, drawn as the page shows the
 * character, so two made up here sound like one person.
 */
export function CharacterDetail({ name, onDeleted }: Props) {
  const detail = useListeningCharacter(name)
  const [seed] = useState(randomSeed)
  if (detail.error) {
    return <ErrorText error={detail.error} />
  }
  if (detail.data === undefined) {
    return <p className="text-sm text-muted-foreground">加载中…</p>
  }
  const character = detail.data
  if (character === null) {
    return <p className="text-sm text-muted-foreground">无</p>
  }
  return (
    <div className="flex flex-col gap-5">
      <div className="flex items-baseline gap-3">
        <h2 className="text-xl font-semibold">{character.name}</h2>
        <span lang="ja" className="text-muted-foreground">
          {character.kana}
        </span>
        <span className="text-sm text-muted-foreground">{genderNames[character.gender]}</span>
        <span className="text-sm text-muted-foreground">{character.age} 岁</span>
      </div>
      <DetailList>
        {languages.map((language) => (
          <DetailItem key={language} label={`${languageNames[language]}参考音`}>
            <ReferenceRow
              name={character.name}
              language={language}
              text={character.references.find((r) => r.language === language)?.text ?? null}
              prompt={character.prompt}
              seed={seed}
            />
          </DetailItem>
        ))}
        <DetailItem label="prompt">
          {character.prompt ?? <span className="text-muted-foreground">无</span>}
        </DetailItem>
        <DetailItem label="出场">
          <CharacterUsage name={character.name} />
        </DetailItem>
      </DetailList>
      <DeleteCharacterButton name={character.name} onDeleted={onDeleted} />
    </div>
  )
}
