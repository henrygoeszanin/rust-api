mod application; // Módulo que contém a lógica de aplicação
mod domain; // Módulo que contém as entidades de domínio
mod infrastructure; // Módulo que contém a infraestrutura, como repositórios
mod presentation; // Módulo que contém a camada de apresentação, como rotas

use actix_web::{App, HttpServer, web}; // Importa as estruturas do Actix Web para criar o servidor e configurar rotas
use dotenv::dotenv; // Importa dotenv para carregar variáveis de ambiente de um arquivo .env
use sqlx::{Pool, Postgres, postgres::PgPoolOptions}; // Importa sqlx para manipulação do banco de dados PostgreSQL
use std::time::Duration; // Importa Duration para definir tempos de timeout

use infrastructure::task_repository::{TaskRepositoryImpl}; // Importa a implementação do repositório de tarefas
use application::task_service::TaskService; // Importa o serviço de tarefas
use presentation::routes::config; // Importa a configuração das rotas

#[actix_web::main] // Macro que define a função principal do Actix Web
async fn main() -> std::io::Result<()> {
    println!("Iniciando aplicação..."); // Imprime uma mensagem de inicialização
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set"); // Obtém a URL do banco de dados da variável de ambiente

    println!("Conectando ao banco de dados..."); // Imprime uma mensagem de conexão ao banco de dados

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(5) // Define o número máximo de conexões
        .acquire_timeout(Duration::from_secs(5)) // Define o tempo de timeout para adquirir uma conexão
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("Conexão com o banco de dados estabelecida com sucesso!"); // Imprime uma mensagem de sucesso na conexão
                pool
            },
            Err(e) => {
                eprintln!("Failed to connect to Postgres: {}", e); // Imprime uma mensagem de erro na conexão
                std::process::exit(1); // Encerra o processo com código de erro
            }
        };

    if let Err(e) = pool.acquire().await {
        eprintln!("Failed to acquire a connection from pool: {}", e); // Imprime uma mensagem de erro ao adquirir uma conexão do pool
        std::process::exit(1); // Encerra o processo com código de erro
    }

    // Cria a implementação do repositório
    let task_repository = TaskRepositoryImpl { pool };

    // Cria o serviço de Task, injetando o repositório
    let task_service = TaskService::new(task_repository);

    // Cria o estado da aplicação com o serviço de tarefas
    let app_state = presentation::routes::AppState::<TaskRepositoryImpl> { task_service };

    // Configura e inicia o servidor HTTP
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Adiciona o estado da aplicação aos dados do aplicativo
            .configure(config) // Configura as rotas
    })
    .bind("127.0.0.1:8080")?; // Define o endereço e a porta para o servidor

    println!("Aplicação iniciada na porta: 8080"); // Imprime uma mensagem indicando que a aplicação foi iniciada
    server.run().await // Executa o servidor
}