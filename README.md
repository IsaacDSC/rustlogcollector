# Rust Log Collector

## Visão Geral

Este é um projeto de estudo que visa projetar e implementar um coletor de logs de alto desempenho em Rust. O objetivo é aprender conceitos de programação de sistemas enquanto constrói uma ferramenta prática que ingere logs de múltiplas fontes, normaliza-os em eventos estruturados e os encaminha para sistemas de armazenamento ou análise downstream.

## Motivação

Pipelines de observabilidade modernos exigem movimentação de dados eficiente, segura e previsível. O modelo de segurança de memória, baixo overhead e forte modelo de concorrência do Rust o tornam ideal para construir um coletor de logs que possa escalar, minimizar o uso de recursos e evitar armadilhas comuns de tempo de execução.

## Tecnologias Utilizadas

- **Rust** - Linguagem de programação de sistemas
- **Tokio** - Runtime assíncrono para Rust
- **Axum** - Framework web para APIs
- **LZ4** - Compressão de dados
- **Serde** - Serialização e desserialização

## Pré-requisitos

Antes de executar o projeto, certifique-se de ter instalado:

- [Rust](https://www.rust-lang.org/tools/install) (versão 1.75 ou superior)
- Cargo (instalado automaticamente com Rust)

Para verificar se o Rust está instalado corretamente:

```bash
rustc --version
cargo --version
```

## Como Rodar o Projeto

### 1. Clone o repositório

```bash
git clone <url-do-repositorio>
cd rustlogcollector
```

### 2. Compile o projeto

```bash
cargo build
```

Para compilar em modo de otimização (release):

```bash
cargo build --release
```

### 3. Execute o projeto

```bash
cargo run
```

Para executar a versão otimizada:

```bash
cargo run --release
```

### 4. Execute os testes

```bash
cargo test
```

### 5. Verifique a formatação do código

```bash
cargo fmt --check
```

Para formatar automaticamente:

```bash
cargo fmt
```

### 6. Execute o linter

```bash
cargo clippy
```

## Estrutura do Projeto

```
rustlogcollector/
├── src/           # Código fonte
├── examples/      # Exemplos de uso
├── target/        # Artefatos de compilação (gerado automaticamente)
├── Cargo.toml     # Dependências e configurações do projeto
├── Cargo.lock     # Versões exatas das dependências
└── README.md      # Este arquivo
```

## Contribuindo

Este é um projeto de estudo, mas sugestões e contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

## Licença

Este projeto está licenciado sob os termos especificados no arquivo LICENSE.

## Recursos de Aprendizado

- [The Rust Programming Language (Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)

---

**Nota**: Este é um projeto educacional focado no aprendizado de conceitos de programação de sistemas com Rust.
