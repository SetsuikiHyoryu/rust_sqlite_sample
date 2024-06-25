use clap::Parser;
use sqlx::SqlitePool;
use std::{env, error::Error};

use commands::{
    handle_command_add, handle_command_delete, handle_filter_title, handle_no_options, CommandLine,
    Commands,
};

mod commands;
mod database;

/// 动态错误
type DynamicError = Box<dyn Error>;
/// 默认 result
type DefaultResult = Result<(), DynamicError>;

#[tokio::main]
async fn main() -> DefaultResult {
    dotenvy::dotenv()?;
    let pool = handle_connect_database().await?;
    init_poem_table(&pool).await?;
    handle_command_line(&pool).await;
    Ok(())
}

async fn handle_connect_database() -> Result<SqlitePool, DynamicError> {
    let database_url = env::var("DATABASE_FILE")?;
    let database_url = env::current_dir()?.join(database_url);
    let database_url = format!("sqlite://{}", database_url.display());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

async fn init_poem_table(pool: &SqlitePool) -> DefaultResult {
    sqlx::query!(
        r#"
            CREATE TABLE IF NOT EXISTS poems (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                body TEXT NOT NULL
            )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn handle_command_line(pool: &SqlitePool) {
    let command_line = CommandLine::parse();

    if let Some(commands) = command_line.commands {
        match commands {
            Commands::Add(poem) => handle_command_add(pool, &poem).await,
            Commands::Delete(arguments) => handle_command_delete(pool, &arguments).await,
        }

        return;
    }

    if let Some(title) = command_line.title {
        handle_filter_title(pool, &title).await;
        return;
    }

    handle_no_options(pool).await;
}
