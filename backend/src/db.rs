use std::{collections::BTreeMap, str::FromStr};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::task::Task;

pub struct DB {
    pool: SqlitePool,
}

impl DB {
    pub async fn new(mem: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let opts = SqliteConnectOptions::from_str(if mem { ":memory:" } else { "tasks.db" })?
            .create_if_missing(true)
            .to_owned();
        let pool = SqlitePoolOptions::new().connect_with(opts).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                desc TEXT,
                status TEXT NOT NULL,
                due INTEGER NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn store(
        &self,
        task: &Task,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            "INSERT INTO tasks (title, desc, status, due) 
            VALUES($1, $2, $3, $4)",
        )
        .bind(task.title.clone())
        .bind(task.desc.clone())
        .bind(task.status.clone())
        .bind(task.due.clone())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_store() {
        let db = DB::new(true).await.unwrap();
        let res = db
            .store(&Task {
                title: "foo".to_owned(),
                desc: None,
                status: "bar".to_owned(),
                due: 0})
            .await;
        assert!(res.is_ok());
    }
}
