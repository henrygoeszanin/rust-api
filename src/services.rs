use actix_web::{
    web::{
        scope,
        Json,
        Data,
        ServiceConfig
    },
    get,
    post,
    HttpResponse,
    Responder
};

use serde_json::json;

use crate::{schema::CreateTaskSchema,model::TaskModel, AppState};

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check api-rust is running";

    HttpResponse::Ok().json(json!({
         "message": MESSAGE, 
         "status": 200 
        }))
}

#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {

   match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
        body.title,
        body.content
   )
    .fetch_one(&data.db)
    .await {
        Ok(task) => {
            return HttpResponse::Ok().json(json!({
                "message": "Task created successfully",
                "status": 201,
                "task": task
            }));
        },
        Err(e) => {
            eprintln!("Failed to create task: {}", e);
            return HttpResponse::InternalServerError().json(
                json!({
                    "message": e.to_string(),
                    "status": 500
                })
            );
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(health_checker)
            .service(create_task)
    );
}