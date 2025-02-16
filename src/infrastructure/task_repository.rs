use crate::domain::task::Task;
use sqlx::{Postgres, Pool};
use uuid::Uuid; // adicione se ainda não estiver importado

#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, title: String, content: String) -> Result<Task, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<Task, sqlx::Error>;
    async fn update(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error>;
}

#[derive(Clone)]
pub struct TaskRepositoryImpl {
    pub pool: Pool<Postgres>,
}

#[async_trait::async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create(&self, title: String, content: String) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as!(
            Task,
            "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at",
            title,
            content
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(task)
    }

    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as!(
            Task,
            "SELECT id, title, content, created_at FROM tasks"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(tasks)
    }

    async fn delete(&self, id: Uuid) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as!(
            Task,
            "DELETE FROM tasks WHERE id = $1 RETURNING id, title, content, created_at",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(task)
    }

    async fn update(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as!(
            Task,
            "UPDATE tasks SET title = COALESCE($2, title), content = COALESCE($3, content) WHERE id = $1 RETURNING id, title, content, created_at",
            id,
            title,
            content
        )
        .fetch_optional(&self.pool)
        .await?;

        match task {
            Some(task) => Ok(task),
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}