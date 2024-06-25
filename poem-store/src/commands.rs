use core::panic;

use clap::{Args, Parser, Subcommand};
use sqlx::SqlitePool;

use crate::database::{add_poem, delete_poem, find_all_poem, find_poem_by_title};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CommandLine {
    /// 诗词标题
    #[arg(short, long)]
    pub title: Option<String>,

    /// 子命令
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

/// 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 添加诗词
    Add(Poem),

    /// 删除诗词
    Delete(DeleteArguments),
}

/// 诗词
#[derive(Debug, Args)]
pub struct Poem {
    /// 标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 正文
    pub body: String,
}

/// 删除诗词的参数
#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeleteArguments {
    /// 诗词编号
    #[arg(long)]
    pub id: Option<isize>,

    /// 诗词标题
    #[arg(short, long)]
    pub title: Option<String>,
}

pub async fn handle_command_add(pool: &SqlitePool, poem: &Poem) {
    if find_poem_by_title(pool, &poem.title).await.is_ok() {
        panic!("标题为『{}』的诗词已存在。", poem.title);
    }

    match add_poem(pool, poem).await {
        Ok(row_id) => {
            println!(
                "\n添加诗词成功，编号：{}，标题：{}，作者：{}。",
                row_id, poem.title, poem.author
            )
        }

        Err(error) => {
            println!("添加诗词失败，错误信息：\n{error}")
        }
    }
}

pub async fn handle_command_delete(pool: &SqlitePool, arguments: &DeleteArguments) {
    match delete_poem(pool, arguments).await {
        Ok(..) => {
            if let Some(id) = arguments.id {
                println!("根据编号 {id} 删除了诗词。");
                return;
            }

            if let Some(title) = &arguments.title {
                println!("根据标题『{title}』删除了诗词。");
            }
        }

        Err(error) => {
            println!("删除诗词失败，错误信息：\n{error}")
        }
    }
}

pub async fn handle_filter_title(pool: &SqlitePool, title: &str) {
    match find_poem_by_title(pool, title).await {
        Ok(poem) => {
            println!("{}", poem);
        }

        Err(error) => {
            println!("查找诗词失败，错误信息：\n{error}");
        }
    }
}

pub async fn handle_no_options(pool: &SqlitePool) {
    match find_all_poem(pool).await {
        Ok(poems) => {
            poems.iter().for_each(|poem| {
                println!("=================================");
                println!("{}", poem);
            });

            println!("=================================");
        }

        Err(error) => {
            println!("查找诗词失败，错误信息：\n{error}");
        }
    }
}
