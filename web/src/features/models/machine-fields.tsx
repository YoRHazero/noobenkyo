import { useId, type ReactNode } from 'react'
import { Input } from '@/components/ui/input'
import type { MachineDraft } from '@/features/models/models-draft'
import { PrefixList } from '@/features/models/prefix-list'

type Props = {
  machine: MachineDraft
  onChange: (change: Partial<Omit<MachineDraft, 'key'>>) => void
}

type TextKey = 'ssh' | 'models' | 'qwenTts' | 'llamaCpp' | 'llamaPython'

/**
 * How to reach the machine and where to look on it. llama.cpp runs on
 * qwen-tts's python when there is one, so its own only shows without.
 */
export function MachineFields({ machine, onChange }: Props) {
  const text = (key: TextKey, label: string, placeholder: string) => (
    <TextField
      label={label}
      placeholder={placeholder}
      value={machine[key]}
      onChange={(value) => onChange({ [key]: value })}
    />
  )
  return (
    <div className="grid grid-cols-[8rem_1fr] items-center gap-x-3 gap-y-2 text-sm">
      {text('ssh', 'ssh', 'user@host 或 ssh 别名')}
      <Row label="命令前缀">
        <PrefixList prefixes={machine.prefixes} onChange={(prefixes) => onChange({ prefixes })} />
      </Row>
      {text('models', '模型文件夹', '/home/you/models')}
      {text('qwenTts', 'qwen-tts 目录', '/home/you/opt/qwen-tts')}
      {text('llamaCpp', 'llama.cpp 目录', '/home/you/opt/llama.cpp')}
      {machine.qwenTts.trim() === '' && text('llamaPython', 'llama.cpp python', '/usr/bin/python3')}
    </div>
  )
}

type TextFieldProps = {
  label: string
  placeholder: string
  value: string
  onChange: (value: string) => void
}

function TextField({ label, placeholder, value, onChange }: TextFieldProps) {
  const id = useId()
  return (
    <>
      <label htmlFor={id} className="text-muted-foreground">
        {label}
      </label>
      <Input
        id={id}
        placeholder={placeholder}
        value={value}
        className="h-8"
        onChange={(event) => onChange(event.target.value)}
      />
    </>
  )
}

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <>
      <span className="text-muted-foreground">{label}</span>
      <div className="min-w-0">{children}</div>
    </>
  )
}
