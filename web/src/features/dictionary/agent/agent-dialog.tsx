import type { ContentFilter } from '@/bindings'
import { DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { FillSentencesForm } from '@/features/dictionary/agent/fill-sentences-form'
import { WordListForm } from '@/features/dictionary/agent/word-list-form'

type Props = {
  filter: ContentFilter
}

/**
 * Work for an agent on the dictionary: each task copies its starting
 * prompt. What the agent writes comes back through the inbox, into the
 * import dialog.
 */
export function AgentDialog({ filter }: Props) {
  return (
    <DialogContent aria-describedby={undefined}>
      <DialogHeader>
        <DialogTitle>交给 agent</DialogTitle>
      </DialogHeader>
      <Tabs defaultValue="fill">
        <TabsList>
          <TabsTrigger value="fill">补例句</TabsTrigger>
          <TabsTrigger value="list">生成词表</TabsTrigger>
        </TabsList>
        <TabsContent value="fill" className="pt-2">
          <FillSentencesForm filter={filter} />
        </TabsContent>
        <TabsContent value="list" className="pt-2">
          <WordListForm />
        </TabsContent>
      </Tabs>
    </DialogContent>
  )
}
