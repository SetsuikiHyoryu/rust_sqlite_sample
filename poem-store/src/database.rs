use sqlx::SqlitePool;
use std::fmt::Display;

use crate::{
    commands::{DeleteArguments, Poem},
    DefaultResult, DynamicError,
};

/// 数据库中的诗词
pub struct DatabasePoem {
    /// 编号
    pub id: i64,
    /// 标题
    pub title: String,
    /// 作者
    pub author: String,
    /// 正文
    pub body: String,
}

impl Display for DatabasePoem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{} {}\n\n{}\n\n{}\n",
            self.id, self.title, self.author, self.body
        )
    }
}

pub async fn add_poem(pool: &SqlitePool, poem: &Poem) -> Result<i64, DynamicError> {
    let row_id = sqlx::query!(
        r#"
            INSERT INTO poems (title, author, body)
            VALUES (?1, ?2, ?3)
        "#,
        poem.title,
        poem.author,
        poem.body
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(row_id)
}

pub async fn delete_poem(pool: &SqlitePool, arguments: &DeleteArguments) -> DefaultResult {
    let id = arguments.id.unwrap_or(-1) as i32;
    let default_title = "".to_string();
    let title = arguments.title.as_ref().unwrap_or(&default_title);

    sqlx::query!(
        r#"
            DELETE FROM poems
            WHERE id == ?1 OR title == ?2
        "#,
        id,
        title
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_poem_by_title(
    pool: &SqlitePool,
    title: &str,
) -> Result<DatabasePoem, DynamicError> {
    let poem = sqlx::query!(
        r#"
            SELECT * FROM poems
            WHERE title == ?1
        "#,
        title
    )
    .fetch_one(pool)
    .await?;

    Ok(DatabasePoem {
        id: poem.id,
        title: poem.title,
        author: poem.author,
        body: poem.body,
    })
}

pub async fn find_all_poem(pool: &SqlitePool) -> Result<Vec<DatabasePoem>, DynamicError> {
    let poems = sqlx::query!(r#"SELECT * FROM poems"#)
        .fetch_all(pool)
        .await?;

    let poems = poems
        .iter()
        .map(|poem| DatabasePoem {
            id: poem.id,
            title: poem.title.clone(),
            author: poem.author.clone(),
            body: poem.body.clone(),
        })
        .collect::<Vec<DatabasePoem>>();

    Ok(poems)
}
