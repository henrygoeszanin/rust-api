mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

use infrastructure::task_repository::{TaskRepositoryImpl};
use application::task_service::TaskService;
use presentation::routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando aplicação...");
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Conectando ao banco de dados...");

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("Conexão com o banco de dados estabelecida com sucesso!");
                pool
            },
            Err(e) => {
                eprintln!("Failed to connect to Postgres: {}", e);
                std::process::exit(1);
            }
        };

    if let Err(e) = pool.acquire().await {
        eprintln!("Failed to acquire a connection from pool: {}", e);
        std::process::exit(1);
    }

    // Cria a implementação do repositório
    let task_repository = TaskRepositoryImpl { pool };

    // Cria o serviço de Task, injetando o repositório
    let task_service = TaskService::new(task_repository);

    let app_state = presentation::routes::AppState { task_service };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(config)
    })
    .bind("127.0.0.1:8080")?;

    println!("Aplicação iniciada na porta: 8080");
    server.run().await
}