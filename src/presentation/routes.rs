use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::application::task_service::TaskService;
use crate::infrastructure::task_repository::TaskRepositoryImpl;

#[derive(Clone)]
pub struct AppState {
    pub task_service: TaskService<TaskRepositoryImpl>,
}

#[derive(Deserialize)]
pub struct CreateTaskDto {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskDto {
    pub title: Option<String>,
    pub content: Option<String>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthchecker", web::get().to(health_checker))
            .route("/task", web::post().to(create_task))
            .route("/task", web::get().to(get_tasks))
            .route("/task/{id}", web::delete().to(delete_task))
            .route("/task/{id}", web::patch().to(update_task))
    );
}

async fn health_checker() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Health check api-rust is running",
        "status": 200
    }))
}

async fn create_task(
    dto: web::Json<CreateTaskDto>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = data
        .task_service
        .create_task(dto.title.clone(), dto.content.clone())
        .await;
    
    match result {
        Ok(task) => HttpResponse::Created().json(serde_json::json!({
            "message": "Task created successfully",
            "status": 201,
            "task": task
        })),
        Err(e) => {
            eprintln!("Failed to create task: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let result = data.task_service.get_tasks().await;
    match result {
        Ok(tasks) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Tasks retrieved successfully",
            "status": 200,
            "tasks": tasks
        })),
        Err(e) => {
            eprintln!("Failed to get tasks: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

async fn delete_task(
    id: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = id.into_inner();
    let result = data.task_service.delete_task(id).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Task deleted successfully",
            "status": 200,
            "task": task
        })),
        Err(e) => {
            eprintln!("Failed to delete task: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

async fn update_task(
    id: web::Path<Uuid>,
    web::Json(dto): web::Json<UpdateTaskDto>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = id.into_inner();
    let result = data.task_service.update_task(id, dto.title.clone(), dto.content.clone()).await;
    match result {
        Ok(task) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Task updated successfully",
            "status": 200,
            "task": task
        })),
        Err(e) => {
            eprintln!("Failed to update task: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}