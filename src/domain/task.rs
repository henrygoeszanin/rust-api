// Importa as bibliotecas necessárias para serialização e deserialização de dados
use serde::{Deserialize, Serialize};
// Importa a biblioteca chrono para manipulação de datas e horas
use chrono::{DateTime, Utc};
// Importa a biblioteca uuid para geração e manipulação de UUIDs
use uuid::Uuid;

// Define a estrutura Task com atributos que serão serializados e deserializados
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    // Identificador único da tarefa
    pub id: Uuid,
    // Título da tarefa
    pub title: String,
    // Conteúdo ou descrição da tarefa
    pub content: String,
    // Data e hora de criação da tarefa
    pub created_at: DateTime<Utc>,
}