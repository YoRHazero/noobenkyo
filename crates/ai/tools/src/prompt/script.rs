use serde::Deserialize;
use uuid::Uuid;

/// A script to write, as chosen in the generation form: a new episode of
/// the world, or more of one of its episodes.
#[derive(Clone, Debug, Deserialize, specta::Type)]
pub struct ScriptTask {
    pub world: String,
    /// The episode to continue; `None` for a new episode.
    pub appending: Option<Uuid>,
    /// The episodes that come before, in the order of the story. Empty
    /// leaves the choice to the agent.
    pub previous: Vec<Uuid>,
    /// What happens in it. Blank leaves it to the agent.
    pub premise: String,
}

/// The prompt for writing the script and sending it to the app.
pub fn script_prompt(task: &ScriptTask) -> String {
    let goal = match task.appending {
        None => format!("给听力的世界「{}」写新的一集剧本", task.world),
        Some(episode) => format!("续写听力的世界「{}」里的一集（{episode}）", task.world),
    };
    let premise = match task.premise.trim() {
        "" => "引子：没有指定，自己按世界设定和之前的剧情想一个。".to_owned(),
        premise => format!("引子：\n{premise}"),
    };
    let previous = if task.previous.is_empty() {
        "前序集：没有指定，自己判断要参考这个世界里的哪些集。".to_owned()
    } else {
        let uuids: Vec<String> = task.previous.iter().map(Uuid::to_string).collect();
        format!("前序集，按这个顺序：\n{}", uuids.join("\n"))
    };
    format!(
        "用 noobenkyo 命令行{goal}，送进 app。用法先运行 `noobenkyo listening guide` 看。\n\n{premise}\n\n{previous}"
    )
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::{ScriptTask, script_prompt};

    fn uuid(text: &str) -> Uuid {
        text.parse().unwrap()
    }

    #[test]
    fn a_new_episode_lists_the_chosen_episodes_in_order() {
        let prompt = script_prompt(&ScriptTask {
            world: "星见町".into(),
            appending: None,
            previous: vec![
                uuid("9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b"),
                uuid("4f2a6b7c-8d9e-4f0a-9b1c-2d3e4f5a6b7c"),
            ],
            premise: "  美咲和健二在祭典上走散。\n最后在神社的台阶上遇到。\n".into(),
        });
        assert_eq!(
            prompt,
            "用 noobenkyo 命令行给听力的世界「星见町」写新的一集剧本，送进 app。用法先运行 `noobenkyo listening guide` 看。

引子：
美咲和健二在祭典上走散。
最后在神社的台阶上遇到。

前序集，按这个顺序：
9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b
4f2a6b7c-8d9e-4f0a-9b1c-2d3e4f5a6b7c"
        );
    }

    #[test]
    fn what_is_left_open_is_the_agents_to_judge() {
        let prompt = script_prompt(&ScriptTask {
            world: "星见町".into(),
            appending: Some(uuid("9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b")),
            previous: Vec::new(),
            premise: " \n".into(),
        });
        assert_eq!(
            prompt,
            "用 noobenkyo 命令行续写听力的世界「星见町」里的一集（9c1e2f3a-5b6d-4e7f-8a9b-0c1d2e3f4a5b），送进 app。用法先运行 `noobenkyo listening guide` 看。

引子：没有指定，自己按世界设定和之前的剧情想一个。

前序集：没有指定，自己判断要参考这个世界里的哪些集。"
        );
    }
}
