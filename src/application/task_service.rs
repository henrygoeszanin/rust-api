use crate::domain::task::Task;
use crate::infrastructure::task_repository::TaskRepository;

#[derive(Clone)]
pub struct TaskService<R: TaskRepository + Clone> {
    repository: R,
}

impl<R: TaskRepository + Clone> TaskService<R> {
    pub fn new(repository: R) -> Self {
        TaskService { repository }
    }

    pub async fn create_task(&self, title: String, content: String) -> Result<Task, sqlx::Error> {
        self.repository.create(title, content).await
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        self.repository.get_all().await
    }
}