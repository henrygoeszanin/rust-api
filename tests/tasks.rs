use uuid::Uuid;
use chrono::Utc;
use rust_api::presentation::routes::{AppState};
use rust_api::application::task_service::TaskService;
use rust_api::domain::task::Task;
use rust_api::infrastructure::task_repository::TaskRepository;

#[derive(Clone)] // Added derive Clone
struct MockTaskRepository;

#[async_trait::async_trait]
impl TaskRepository for MockTaskRepository {
    async fn create(&self, title: String, content: String) -> Result<Task, sqlx::Error> {
        Ok(Task {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: Utc::now(),
        })
    }

    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error> {
        Ok(vec![])
    }

    async fn delete(&self, id: Uuid) -> Result<Task, sqlx::Error> {
        Ok(Task {
            id,
            title: "deleted".into(),
            content: "deleted".into(),
            created_at: Utc::now(),
        })
    }

    async fn update(&self, id: Uuid, title: Option<String>, content: Option<String>) -> Result<Task, sqlx::Error> {
        Ok(Task {
            id,
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            created_at: Utc::now(),
        })
    }
}

#[actix_web::test]
async fn test_create_task_service() {
    let mock_repo = MockTaskRepository;
    let task_service = TaskService::new(mock_repo);
    let app_state = AppState::<MockTaskRepository> { task_service };

    let title = "Test Task".to_string();
    let content = "This is a test task.".to_string();
    let result = app_state.task_service.create_task(title.clone(), content.clone()).await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.title, title);
    assert_eq!(task.content, content);
}

#[actix_web::test]
async fn test_get_tasks_service() {
    let mock_repo = MockTaskRepository;
    let task_service = TaskService::new(mock_repo);
    let app_state = AppState::<MockTaskRepository> { task_service };

    let result = app_state.task_service.get_tasks().await;
    assert!(result.is_ok());
    let tasks = result.unwrap();
    // Our mock returns an empty vector
    assert!(tasks.is_empty());
}

#[actix_web::test]
async fn test_delete_task_service() {
    let mock_repo = MockTaskRepository;
    let task_service = TaskService::new(mock_repo);
    let app_state = AppState::<MockTaskRepository> { task_service };

    let id = Uuid::new_v4();
    let result = app_state.task_service.delete_task(id).await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, id);
    assert_eq!(task.title, "deleted");
    assert_eq!(task.content, "deleted");
}

#[actix_web::test]
async fn test_update_task_service() {
    let mock_repo = MockTaskRepository;
    let task_service = TaskService::new(mock_repo);
    let app_state = AppState::<MockTaskRepository> { task_service };

    let id = Uuid::new_v4();
    let new_title = Some("Updated Title".to_string());
    let new_content = Some("Updated Content".to_string());
    let result = app_state.task_service.update_task(id, new_title.clone(), new_content.clone()).await;
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, id);
    assert_eq!(task.title, new_title.unwrap());
    assert_eq!(task.content, new_content.unwrap());
}