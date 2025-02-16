use crate::domain::task::Task; // Importa a estrutura Task do módulo de domínio
use sqlx::{Postgres, Pool}; // Importa sqlx para manipulação do banco de dados PostgreSQL
use uuid::Uuid; // Importa Uuid para gerar e manipular UUIDs

#[async_trait::async_trait] // Define um trait assíncrono para o repositório de tarefas
pub trait TaskRepository: Send + Sync {
    async fn create(&self, title: String, content: String) -> Result<Task, sqlx::Error>; // Método para criar uma nova tarefa
    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error>; // Método para obter todas as tarefas
    async fn delete(&self, id: Uuid) -> Result<Task, sqlx::Error>; // Método para deletar uma tarefa
    async fn update(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error>; // Método para atualizar uma tarefa
}

#[derive(Clone)] // Permite que TaskRepositoryImpl seja clonável
pub struct TaskRepositoryImpl {
    pub pool: Pool<Postgres>, // Pool de conexões com o banco de dados PostgreSQL
}

#[async_trait::async_trait] // Implementa o trait TaskRepository para TaskRepositoryImpl
impl TaskRepository for TaskRepositoryImpl {
    async fn create(&self, title: String, content: String) -> Result<Task, sqlx::Error> {
        // Cria uma nova tarefa no banco de dados
        let task = sqlx::query_as!(
            Task,
            "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING id, title, content, created_at",
            title,
            content
        )
        .fetch_one(&self.pool) // Executa a query e obtém a tarefa criada
        .await?;
        Ok(task) // Retorna a tarefa criada
    }

    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error> {
        // Obtém todas as tarefas do banco de dados
        let tasks = sqlx::query_as!(
            Task,
            "SELECT id, title, content, created_at FROM tasks"
        )
        .fetch_all(&self.pool) // Executa a query e obtém todas as tarefas
        .await?;
        Ok(tasks) // Retorna a lista de tarefas
    }

    async fn delete(&self, id: Uuid) -> Result<Task, sqlx::Error> {
        // Deleta uma tarefa do banco de dados
        let task = sqlx::query_as!(
            Task,
            "DELETE FROM tasks WHERE id = $1 RETURNING id, title, content, created_at",
            id
        )
        .fetch_one(&self.pool) // Executa a query e obtém a tarefa deletada
        .await?;
        Ok(task) // Retorna a tarefa deletada
    }

    async fn update(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error> {
        // Atualiza uma tarefa no banco de dados
        let task = sqlx::query_as!(
            Task,
            "UPDATE tasks SET title = COALESCE($2, title), content = COALESCE($3, content) WHERE id = $1 RETURNING id, title, content, created_at",
            id,
            title,
            content
        )
        .fetch_optional(&self.pool) // Executa a query e obtém a tarefa atualizada, se existir
        .await?;

        match task {
            Some(task) => Ok(task), // Retorna a tarefa atualizada
            None => Err(sqlx::Error::RowNotFound), // Retorna um erro se a tarefa não for encontrada
        }
    }
}