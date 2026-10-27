//! The starting prompts reach the frontend as agent writes them.

use agent::{FillSentencesTask, ScriptTask, WordListTask};
use dictionary::ProgressState;
use noobenkyo_core::commands::prompts::{
    agent_fill_sentences_prompt, agent_script_prompt, agent_word_list_prompt,
};

use crate::support::WORLD;

#[test]
fn each_prompt_starts_the_agent_on_its_guide() {
    let script = agent_script_prompt(ScriptTask {
        world: WORLD.into(),
        appending: None,
        previous: Vec::new(),
        premise: "美咲和健二在祭典上走散。".into(),
    });
    assert!(script.contains("「星见町」写新的一集剧本"));
    assert!(script.contains("`noobenkyo listening guide`"));

    let sentences = agent_fill_sentences_prompt(FillSentencesTask {
        keyword: None,
        progress: ProgressState::Any,
        limit: 20,
        per_sense: 2,
    });
    assert!(sentences.contains("`noobenkyo dictionary guide`"));
    assert!(sentences.contains("--limit 20`"));

    let words = agent_word_list_prompt(WordListTask {
        topic: "便利店购物".into(),
        count: 50,
        per_sense: 2,
    });
    assert!(words.contains("话题：便利店购物"));
}
