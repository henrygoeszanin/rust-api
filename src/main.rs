// Importa os módulos internos do projeto.
mod services;
mod schema;
mod model;

// Importa os componentes necessários do Actix Web.
use actix_web::{App, HttpServer, web};

// Importa o utilitário dotenv para carregar variáveis de ambiente.
use dotenv::dotenv;

// Importa os tipos e traits necessários do sqlx para conexão com PostgreSQL.
use sqlx::{
    Postgres,
    Pool,
    postgres::PgPoolOptions,
};

use std::time::Duration; // Importa a duração para configurar o timeout.

// Estrutura que mantém o estado da aplicação, incluindo a conexão com o banco de dados.
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando aplicação...");
    // Carrega as variáveis de ambiente do arquivo .env (se existir).
    dotenv().ok();

    // Recupera a URL do banco de dados das variáveis de ambiente.
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    
    println!("Conectando ao banco de dados...");


    // Cria um pool de conexões para o PostgreSQL utilizando a URL obtida.
    let pool = match PgPoolOptions::new()
        .max_connections(5) // Define o número máximo de conexões simultâneas.
        .acquire_timeout(Duration::from_secs(5)) // Define um timeout de 5 segundos para a conexão.
        .connect(&database_url)
        .await {
            Ok(pool) => { // Pool criado com sucesso.
                println!("Conexão com o banco de dados estabelecida com sucesso!");
                pool
            },
            Err(e) => { // Em caso de erro, imprime a mensagem e encerra a aplicação.
                eprintln!("Failed to connect to Postgres: {}", e);
                std::process::exit(1);
            }
        };

    // Tenta adquirir uma conexão do pool para garantir que o banco de dados esteja acessível.
    if let Err(e) = pool.acquire().await {
        eprintln!("Failed to acquire a connection from pool: {}", e);
        std::process::exit(1); // Encerra a aplicação se não conseguir conexão.
    }
    
    // Cria e configura o servidor HTTP do Actix Web.
    let server = HttpServer::new(move || {
        App::new()
            // Insere o estado da aplicação com a conexão do pool no contexto do Actix.
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
            }))
            // Configura as rotas e serviços definidos no módulo services.
            .configure(services::config)
    })
    // Define o endereço e porta onde o servidor irá escutar.
    .bind("127.0.0.1:8080")?;

    println!("Aplicação iniciada na porta: 8080");
    // Inicia o servidor.
    server.run().await
}