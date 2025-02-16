use actix_web::{web, HttpResponse, Responder}; // Importa estruturas do Actix Web para lidar com requisições HTTP
use serde::Deserialize; // Importa Deserialize para deserializar dados JSON
use uuid::Uuid; // Importa Uuid para gerar e manipular UUIDs
use crate::infrastructure::task_repository::TaskRepositoryImpl; // Importa a implementação do repositório de tarefas
use crate::application::task_service::TaskService; // Importa o serviço de tarefas
use crate::infrastructure::task_repository::TaskRepository; // Importa o trait do repositório de tarefas

#[derive(Clone)] // Permite que AppState seja clonável
pub struct AppState<R: TaskRepository + Clone> {  
    pub task_service: TaskService<R>, // Contém o serviço de tarefas
}

#[derive(Deserialize)] // Permite deserializar dados JSON para CreateTaskDto
pub struct CreateTaskDto {
    pub title: String, // Título da tarefa
    pub content: String, // Conteúdo da tarefa
}

#[derive(Deserialize)] // Permite deserializar dados JSON para UpdateTaskDto
pub struct UpdateTaskDto {
    pub title: Option<String>, // Novo título da tarefa (opcional)
    pub content: Option<String>, // Novo conteúdo da tarefa (opcional)
}

// Configura as rotas da aplicação
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthchecker", web::get().to(health_checker)) // Rota para verificação de saúde
            .route("/task", web::post().to(create_task)) // Rota para criar uma tarefa
            .route("/task", web::get().to(get_tasks)) // Rota para obter todas as tarefas
            .route("/task/{id}", web::delete().to(delete_task)) // Rota para deletar uma tarefa
            .route("/task/{id}", web::patch().to(update_task)) // Rota para atualizar uma tarefa
    );
}

// Função para verificação de saúde da aplicação
pub async fn health_checker() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Health check api-rust is running",
        "status": 200
    }))
}

// Função para criar uma nova tarefa
async fn create_task(
    dto: web::Json<CreateTaskDto>, // Dados da tarefa recebidos no corpo da requisição
    data: web::Data<AppState<TaskRepositoryImpl>>, // Estado da aplicação
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
            eprintln!("Failed to create task: {}", e); // Imprime mensagem de erro no console
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

// Função para obter todas as tarefas
async fn get_tasks(data: web::Data<AppState<TaskRepositoryImpl>>) -> impl Responder {
    let result = data.task_service.get_tasks().await;
    match result {
        Ok(tasks) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Tasks retrieved successfully",
            "status": 200,
            "tasks": tasks
        })),
        Err(e) => {
            eprintln!("Failed to get tasks: {}", e); // Imprime mensagem de erro no console
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

// Função para deletar uma tarefa
async fn delete_task(
    id: web::Path<Uuid>, // ID da tarefa recebido na URL
    data: web::Data<AppState<TaskRepositoryImpl>>, // Estado da aplicação
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
            eprintln!("Failed to delete task: {}", e); // Imprime mensagem de erro no console
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}

// Função para atualizar uma tarefa
async fn update_task(
    id: web::Path<Uuid>, // ID da tarefa recebido na URL
    web::Json(dto): web::Json<UpdateTaskDto>, // Dados da tarefa recebidos no corpo da requisição
    data: web::Data<AppState<TaskRepositoryImpl>>, // Estado da aplicação
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
            eprintln!("Failed to update task: {}", e); // Imprime mensagem de erro no console
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": e.to_string(),
                "status": 500
            }))
        }
    }
}