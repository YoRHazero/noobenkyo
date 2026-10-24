use serde::Deserialize;

/// A new word list on a topic, written down to its sentences.
#[derive(Clone, Debug, Deserialize, specta::Type)]
pub struct WordListTask {
    /// Blank leaves the topic to the agent.
    pub topic: String,
    /// How many words.
    pub count: u32,
    /// How many sentences each sense gets.
    pub per_sense: u32,
}

/// The prompt for writing the list and sending it to the app.
pub fn word_list_prompt(task: &WordListTask) -> String {
    let topic = match task.topic.trim() {
        "" => "没有指定，自己选一个日常的话题",
        topic => topic,
    };
    format!(
        "用 noobenkyo 命令行生成一份日语词表，送进 app。用法先运行 `noobenkyo dictionary guide` 看。\n\n话题：{topic}\n个数：{}\n只列动词、形容词、副词和名词，写到例句，每个义项 {} 条例句。词库里已有的词不要。",
        task.count, task.per_sense
    )
}

#[cfg(test)]
mod tests {
    use super::{WordListTask, word_list_prompt};

    #[test]
    fn the_topic_and_counts_are_given() {
        let prompt = word_list_prompt(&WordListTask {
            topic: "便利店购物".into(),
            count: 50,
            per_sense: 2,
        });
        assert_eq!(
            prompt,
            "用 noobenkyo 命令行生成一份日语词表，送进 app。用法先运行 `noobenkyo dictionary guide` 看。

话题：便利店购物
个数：50
只列动词、形容词、副词和名词，写到例句，每个义项 2 条例句。词库里已有的词不要。"
        );
    }

    #[test]
    fn a_blank_topic_is_the_agents_to_choose() {
        let prompt = word_list_prompt(&WordListTask {
            topic: " ".into(),
            count: 10,
            per_sense: 1,
        });
        assert!(prompt.contains("话题：没有指定，自己选一个日常的话题\n"));
    }
}
