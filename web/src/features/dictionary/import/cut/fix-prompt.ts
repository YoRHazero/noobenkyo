/** Between the turns of a dialogue: a full-width space. */
const turnGap = '\u3000'

/** What the AI is asked to do, and the entry format it has to follow. */
const instructions = `下面是一份日语单词表里有警告的几行，格式是 JSON Lines：一行一个 JSON 对象。请修正"警告"里列出的问题，并把每一行补全。

输出：
- 只输出修正、补全后的行，放在一个代码块里，一行一个对象，不要解释。
- 输入几行就输出几行。只有原来一行里混了几个读音或几个义项时，才拆成几行。
- labels、stage、passes、lapses、modified_at 原样保留，不新增。

修正：
- 疑似活用形：写法是活用形的，改成词典形，kana 和例句跟着改；其实就是词典形的，写法不改。
- 未知字段：是某个已有字段写错了的（比如 gloss），并进那个字段；其余的删掉。
- 丢掉的例句：改写成符合"例句的要求"的句子。

补全：每一行都补到有读音、语种、释义和 3 条例句。
- writing：写法，只能包含汉字和假名。
- kana：平假名读音，只能包含平假名和 ー。原来写了读音的，按这个读音补全。
- origin：外来语填原词，按原语言拼写，例：コーヒー → koffie；和制外来语填组成它的原词，例：パソコン → personal computer；日语固有词和混合词不写这个字段。
- stratum：语种，填 和语、汉语、外来语、混种语 之一。
- glosses：中文释义列表，第一个是主释义，要简洁。原来没有释义的，补这个词最常用的读音和义项。
- sentences：例句列表，每条是 {"marked": "带标记的日语句子", "translation": "中文翻译"}，补到 3 条，原有的合格例句保留。

例句的要求：
- 地道、自然，是母语者日常会说的话。
- 可以是两个人的对话，写在一行里：甲：「……」${turnGap}乙：「……」。
- 语境明确：读完整句，能确定目标词在这里是这个义项的意思。
- 不能靠句子猜出目标词：把目标词挖空，换成别的几个词，句子也通顺。
- 几条例句用不同的场景，目标词尽量用不同的活用形。
- 标记：除了甲、乙，所有汉字都标注音，写成 {漢字|かな}；目标词用 [[ ]] 标出，必须恰好出现一次，可以是活用形。
- translation：通顺的中文，翻译整句或整段对话；对话写成 甲：“……”${turnGap}乙：“……”。

例：
{"writing": "諦める", "stratum": "和语", "kana": "あきらめる", "glosses": ["放弃", "死心"], "sentences": [{"marked": "{何度|なんど}{失敗|しっぱい}しても、[[{諦|あきら}めない]]でね。", "translation": "不管失败多少次，都别放弃哦。"}, {"marked": "{雨|あめ}がひどいから、{今日|きょう}のハイキングは[[{諦|あきら}めよう]]か。", "translation": "雨太大了，今天的徒步就放弃吧。"}, {"marked": "甲：「{結局|けっきょく}、{留学|りゅうがく}はどうしたの？」${turnGap}乙：「お{金|かね}が{足|た}りなくて[[{諦|あきら}めた]]よ。」", "translation": "甲：“留学的事最后怎么样了？”${turnGap}乙：“钱不够，放弃了。”"}]}`

/**
 * The prompt for a chat AI to fix and complete cut lines. `warnings` are the
 * check of the cut lines alone, so their line numbers count those lines
 * from 1, as they appear under "原文".
 */
export function fixPrompt(warnings: string[], lines: string[]): string {
  return [
    instructions,
    '',
    '警告（行号是"原文"里的行号）：',
    ...warnings,
    '',
    '原文：',
    '```',
    ...lines,
    '```',
  ].join('\n')
}
