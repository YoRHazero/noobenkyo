# noobenkyo 剧本

## 命令

```
noobenkyo listening guide
noobenkyo listening worlds
noobenkyo listening episodes 星见町
noobenkyo listening context 星见町 --after 0b6f6a8e-3c1d-4f0e-9a43-2a1d5c7e9b10 --after 5d2c9a14-7e3b-4c8f-b1a6-9f0e2d4c8a37
noobenkyo listening context 星见町 --append 5d2c9a14-7e3b-4c8f-b1a6-9f0e2d4c8a37
noobenkyo listening check ep.json
noobenkyo listening send ep.json --world 星见町
noobenkyo listening send ep.json --append 5d2c9a14-7e3b-4c8f-b1a6-9f0e2d4c8a37
```

- **guide**：打印这份说明。
- **worlds**：所有世界，JSON 数组，每项是 name（世界名）、episodes（有几集）。
- **episodes 世界**：这个世界的集，按集数排，JSON 数组。每项是 uuid、number（集数）、title、summary（梗概）、created_at、lines（行数）、char_count（日语和中文的字数，读音标记也算）、missing_ja 和 missing_zh（缺几行音频）。
- **context 世界**：写剧本要用的上下文，JSON 对象：
  - world、setting：世界名和设定。
  - mains：主要角色，每个是 name、kana（名字的读音）、gender（female 或 male）、age、description（在这个世界里的身份）。
  - predecessors：`--after` 选的集的全文，按写的顺序，也就是剧情的先后。每集是 number、title、summary、lines（每行 speaker、ja、zh）。`--after` 可以写多次，也可以不写。
  - appending：`--append` 那一集的全文，没写时是 null。续写时用。
  - 世界没有旁白或者没有主要角色时，命令失败。
- **check 文件**：按存剧本的标准检查。通过时没有输出。有问题时每条一行，例如：
  - `title 为空`、`缺少 lines`
  - `第 2 行：speaker 为空`
  - `第 1 行 ja：读音标记 {上手|うわて 没有闭合`

  行号就是 lines 里的第几行，从 1 开始。用户在预览里说"第 12 行"，也是这个行号。
- **send 文件**：送进 app 的剧本预览，用户确认以后才存。`--world 世界` 存成这个世界的新一集，`--append 集的uuid` 续写这一集，两个只能写一个。送之前先检查剧本，再查世界在不在、旁白和主要角色齐不齐、续写的那一集在不在；有问题就不送，问题和 check 一样列出来。命令拿不到 app 的回答。成功时输出"已送进 inbox"。

文件写成 `-` 时读标准输入。退出码：0 是完成；1 是内容有问题，问题打在标准输出；2 是命令没能运行（例如找不到数据、世界不存在），原因打在标准错误。

## 格式

```json
{ "title": "…", "summary": "…", "lines": [ { "speaker": "…", "ja": "…", "zh": "…" } ] }
```

- title、summary 是不为空的字符串。
- lines 至少一行；每一行的 speaker、ja、zh 都是不为空的字符串。
- 多出来的字段会被忽略。
- 续写：lines 只写新的行，接在这一集的末尾；summary 换成覆盖整集（旧的加新的）的梗概；title 不会用到，但也不能为空，照抄原来的标题。

## 各字段的要求

- **speaker**：
  - 主要角色说话时，原样写 context 里 mains 的 name。
  - 旁白写「旁白」。
  - 其他人随便写一个称呼，例如「店員」「客」；同一个人在整集里用同一个称呼。他们会自动配上世界里次要角色的声音。
- **lines**：
  - 一行只放一个人说的话。叙述交给旁白，单独成行。
  - 人物的言行要符合各自的身份（description）和常识，情绪符合日常，不要夸张。
  - 故事讲完就结束，不要重复已经写过的内容。
- **ja**：
  - 提到主要角色的名字时写读音（mains 的 kana），例如 みさき。
  - 容易念错的地方标读音，写成 `{写法|读音}`，读音只能有假名，例如 `{上手|うわて}`。要标的：这里的读法不是这个写法最常见的读法（上手 うわて、人気 ひとけ、生物 なまもの、一日 ついたち），少见的人名、地名、店名。按最常见的读法念就对的词不用标。
  - 每个标记都要完整；标记以外不能有单独的 `{`、`|`、`}`。
  - 不要写（笑）（小声で）这类括号里的说明，合成语音时会被念出来。语气靠台词本身和标点。
- **zh**：自然的中文，和这一行的日语对应；主要角色的名字写 name（例如 美咲）。
- **title**：中文，简短。
- **summary**：中文，写清楚发生了什么、人物之间的关系有什么变化。以后写续集时会拿它参考。

## 例子

```json
{
  "title": "海边的早晨",
  "summary": "美咲在海边的便利店遇到同学健二，两人约好放学后一起去看灯塔。",
  "lines": [
    { "speaker": "旁白", "ja": "{潮風|しおかぜ}が{吹|ふ}く、{春|はる}の{朝|あさ}。", "zh": "海风吹拂的春日早晨。" },
    { "speaker": "店員", "ja": "いらっしゃいませ。", "zh": "欢迎光临。" },
    { "speaker": "美咲", "ja": "あれ？けんじくん、こんな{時間|じかん}にどうしたの？", "zh": "咦？健二，这么早你怎么在这儿？" },
    { "speaker": "健二", "ja": "{部活|ぶかつ}の{朝練|あされん}だよ。みさきこそ{早|はや}いな。", "zh": "社团晨练啊。美咲你也挺早的嘛。" }
  ]
}
```
