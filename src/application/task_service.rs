use crate::domain::task::Task; // Importa a estrutura Task do módulo de domínio
use crate::infrastructure::task_repository::TaskRepository; // Importa o trait TaskRepository
use uuid::Uuid; // Importa Uuid para gerar e manipular UUIDs

#[derive(Clone)] // Permite que TaskService seja clonável
pub struct TaskService<R: TaskRepository + Clone> {
    repository: R, // Repositório de tarefas
}

impl<R: TaskRepository + Clone> TaskService<R> {
    pub fn new(repository: R) -> Self {
        TaskService { repository } // Cria uma nova instância de TaskService com o repositório fornecido
    }

    pub async fn create_task(&self, title: String, content: String) -> Result<Task, sqlx::Error> {
        // Cria uma nova tarefa usando o repositório
        self.repository.create(title, content).await
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        // Obtém todas as tarefas usando o repositório
        self.repository.get_all().await
    }

    // Métodos adicionados:
    pub async fn delete_task(&self, id: Uuid) -> Result<Task, sqlx::Error> {
        // Deleta uma tarefa usando o repositório
        self.repository.delete(id).await
    }

    pub async fn update_task(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error> {
        // Atualiza uma tarefa usando o repositório
        self.repository.update(id, title, content).await
    }
}