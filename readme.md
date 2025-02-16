# Documentação da Arquitetura Clean/Hexagonal

Esta documentação descreve a arquitetura adotada na aplicação, destacando seu nome, conceitos e estruturas.

## Visão Geral

A aplicação utiliza uma **Clean Architecture** com influência na **Arquitetura Hexagonal**. O principal objetivo é promover o desacoplamento entre as regras de negócio, casos de uso e a infraestrutura, facilitando a manutenção e a testabilidade do código.

## Características e Conceitos

- **Separação de responsabilidades:**  
  - **Domínio:** Define as entidades e regras de negócio.  
  - **Aplicação:** Contém os casos de uso e a lógica que orquestra as operações entre as camadas.  
  - **Infraestrutura:** Implementa os detalhes externos, como acesso ao banco de dados e integrações com bibliotecas externas.  
  - **Apresentação:** Gerencia as interfaces de entrada (ex. endpoints HTTP) e exibe os dados ao usuário.

- **Desacoplamento:**  
  Cada camada funciona de forma independente e se comunica por meio de interfaces/contratos, permitindo a substituição de implementações sem afetar a lógica de negócio.

- **Testabilidade:**  
  Ao isolar as regras de negócio e os casos de uso das demais camadas, torna-se mais simples escrever testes unitários e de integração.

## Por que "Hexagonal"?

O nome "Hexagonal" vem da representação gráfica frequentemente utilizada para ilustrar este padrão arquitetural. Nessa representação, o núcleo da aplicação (com suas regras de negócio) é mostrado como um hexágono, e cada uma de suas faces representa uma "porta" pela qual a aplicação interage com o mundo externo (como interfaces de usuário, banco de dados, serviços externos, etc.). Essa figura destaca a ideia central de que a lógica de negócio está isolada e pode ser acessada e testada através desses pontos de integração, promovendo o desacoplamento.

## Observações sobre Clean Architecture e Arquitetura Hexagonal

Atualmente, não existe uma diferença clara entre Clean Architecture e Clean Architecture Hexagonal. A Clean Architecture aprimorou os conceitos da Arquitetura Hexagonal, incorporando a ideia de portas e adaptadores para isolar o núcleo de domínio dos detalhes externos. Dessa forma, os dois termos são frequentemente usados de forma intercambiável para descrever uma arquitetura que promove:
- **Desacoplamento total:** O domínio não depende de frameworks ou tecnologias externas.
- **Flexibilidade:** Possibilidade de alterar adaptadores (como interfaces ou conexões de banco) sem impactar a lógica de negócio.
- **Testabilidade:** Facilidade em testar o domínio isoladamente.

## Principais Bibliotecas Utilizadas

- **Actix-web:**  
  Framework web assíncrono utilizado para gerenciar os endpoints HTTP e a comunicação com o cliente.

- **SQLx:**  
  ORM assíncrono utilizado para acesso ao PostgreSQL. Possui suporte a migrações e verificação em tempo de compilação das queries.

- **Chrono:**  
  Biblioteca para manipulação de data e hora, utilizada na definição de timestamps nas entidades.

- **Uuid:**  
  Biblioteca para geração e manipulação de identificadores UUID, garantindo identificação única para registros.

- **Serde / Serde JSON:**  
  Utilizado para serialização e deserialização de dados, facilitando o envio e recebimento de informações via JSON.

- **Dotenv:**  
  Permite carregar variáveis de ambiente de um arquivo `.env`, facilitando a configuração da aplicação.

## Considerações Finais

Esta arquitetura promove um código organizado e testável, onde cada camada é responsável por um aspecto específico da aplicação. O desacoplamento entre as camadas facilita futuras manutenções, evoluções e a substituição de tecnologias, mantendo a integridade das regras de negócio.

Esta documentação serve como ponto de referência para novos desenvolvedores e para a manutenção da aplicação.