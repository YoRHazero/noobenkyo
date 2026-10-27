//! The commands and their arguments. One command per tool, grouped by the
//! feature it belongs to.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use dictionary::{Completeness, ContentFilter, ProgressState};
use uuid::Uuid;

/// 给外部 agent 用的命令行：读词库和听力的数据、检查写好的内容、送进 app。先运行 guide。
#[derive(Parser)]
#[command(name = "noobenkyo", version)]
pub struct Cli {
    #[command(subcommand)]
    pub group: Group,
}

#[derive(Subcommand)]
pub enum Group {
    /// 词库：条目的说明、导出、查已有的词、检查、送进导入对话框
    #[command(subcommand)]
    Dictionary(DictionaryCommand),
    /// 听力：剧本的说明、列世界和集、写剧本的上下文、检查、送进 app、读回送进去的剧本
    #[command(subcommand)]
    Listening(ListeningCommand),
}

#[derive(Subcommand)]
pub enum DictionaryCommand {
    /// 条目的说明：格式、写法、例子、流程
    Guide,
    /// 按筛选导出条目，不带进度
    Export(ExportArgs),
    /// 列出文件里已经在词库里的单词（文件是条目格式，- 表示标准输入）
    Existing { file: PathBuf },
    /// 检查条目：通过时没有输出；有问题时每条一行，退出码 1
    Check { file: PathBuf },
    /// 送进 app 的导入对话框，名字取文件名（去掉扩展名）
    Send { file: PathBuf },
}

#[derive(Args)]
pub struct ExportArgs {
    /// 包含在写法、词源、读音或释义里的文字
    #[arg(long)]
    keyword: Option<String>,
    /// 写到哪一层，可以写多次；不写表示不限
    #[arg(long, value_enum)]
    completeness: Vec<CompletenessArg>,
    /// 进度
    #[arg(long, value_enum, default_value_t = ProgressArg::Any)]
    progress: ProgressArg,
    /// 最多导出几行
    #[arg(long)]
    pub limit: Option<u32>,
}

impl ExportArgs {
    pub fn filter(&self) -> ContentFilter {
        ContentFilter {
            text: self.keyword.clone(),
            completeness: self
                .completeness
                .iter()
                .map(|level| level.completeness())
                .collect(),
            progress: self.progress.state(),
        }
    }
}

/// How far a row is written.
#[derive(Clone, Copy, ValueEnum)]
pub enum CompletenessArg {
    /// 只有写法
    WordOnly,
    /// 写到读音
    ReadingOnly,
    /// 有义项没例句
    NoSentence,
    /// 有义项有例句
    WithSentence,
}

impl CompletenessArg {
    fn completeness(self) -> Completeness {
        match self {
            CompletenessArg::WordOnly => Completeness::WordOnly,
            CompletenessArg::ReadingOnly => Completeness::ReadingOnly,
            CompletenessArg::NoSentence => Completeness::NoSentence,
            CompletenessArg::WithSentence => Completeness::WithSentence,
        }
    }
}

/// Progress of a sense.
#[derive(Clone, Copy, ValueEnum)]
pub enum ProgressArg {
    /// 不限
    Any,
    /// 正确次数和档位都是 0
    Untouched,
    /// 学过的
    Touched,
}

impl ProgressArg {
    fn state(self) -> ProgressState {
        match self {
            ProgressArg::Any => ProgressState::Any,
            ProgressArg::Untouched => ProgressState::Untouched,
            ProgressArg::Touched => ProgressState::Touched,
        }
    }
}

#[derive(Subcommand)]
pub enum ListeningCommand {
    /// 剧本的说明：格式、写法、例子、流程
    Guide,
    /// 列出所有世界（JSON）
    Worlds,
    /// 列出一个世界的集，每集带梗概和字数（JSON）
    Episodes { world: String },
    /// 写剧本的上下文：世界、主要角色、前序集和续写那一集的全文（JSON）
    Context {
        world: String,
        /// 前序集的 uuid，可以写多次，写的顺序就是交给模型的顺序
        #[arg(long = "after")]
        predecessors: Vec<Uuid>,
        /// 续写的集的 uuid
        #[arg(long)]
        append: Option<Uuid>,
    },
    /// 检查剧本：通过时没有输出；有问题时每条一行，退出码 1
    Check { file: PathBuf },
    /// 先在本地检查，通过了才送进 app 的剧本预览，名字取文件名（去掉扩展名）
    Send {
        file: PathBuf,
        /// 存成这个世界的新一集
        #[arg(long, required_unless_present = "append", conflicts_with = "append")]
        world: Option<String>,
        /// 续写这一集（uuid）
        #[arg(long)]
        append: Option<Uuid>,
        /// 覆写 inbox 里同名的那一份剧本；改之前先 show
        #[arg(long)]
        replace: bool,
    },
    /// 打印 inbox 里这一份剧本的当前版本，带着用户在预览里的修改
    Show { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The command lines a guide shows: lines that start with `noobenkyo`,
    /// cut before a shell redirection.
    fn command_lines(guide: &str) -> Vec<Vec<&str>> {
        guide
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("noobenkyo "))
            .map(|line| {
                let command = line.split(" > ").next().unwrap_or(line);
                command.split_whitespace().collect()
            })
            .collect()
    }

    #[test]
    fn every_command_in_the_dictionary_guide_parses() {
        assert_every_command_parses(agent_tools::dictionary_guide());
    }

    #[test]
    fn every_command_in_the_listening_guide_parses() {
        assert_every_command_parses(agent_tools::listening_guide());
    }

    fn assert_every_command_parses(guide: &str) {
        let lines = command_lines(guide);
        assert!(!lines.is_empty());
        for args in lines {
            if let Err(error) = Cli::try_parse_from(&args) {
                panic!("{args:?} does not parse: {error}");
            }
        }
    }

    #[test]
    fn a_wrong_command_is_caught() {
        let lines = command_lines("noobenkyo dictionary export --completeness glosses > a.jsonl");
        assert_eq!(
            lines,
            [[
                "noobenkyo",
                "dictionary",
                "export",
                "--completeness",
                "glosses"
            ]]
        );
        assert!(Cli::try_parse_from(&lines[0]).is_err());
    }
}
