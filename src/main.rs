mod compress;
mod store;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use std::env;
use std::process::Stdio;
use store::Store;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Clone)]
struct AppState {
    store: Store,
}

#[derive(Serialize)]
struct LogResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    // Cria o store compartilhado entre as threads
    let store = Store::new(5);

    // Thread 1: Captura logs do programa
    let store_clone = store.clone_handle();
    let log_collector_task = tokio::spawn(async move {
        collect_logs(store_clone).await;
    });

    // Thread 2: Servidor HTTP
    let store_clone = store.clone_handle();
    let http_server_task = tokio::spawn(async move {
        start_http_server(store_clone).await;
    });

    // Aguarda as duas tasks
    let _ = tokio::join!(log_collector_task, http_server_task);
}

async fn collect_logs(store: Store) {
    println!("[LOG COLLECTOR] Iniciando coleta de logs...");

    // Configura o comando via variáveis de ambiente
    // Exemplo: MONITOR_CMD="python3" MONITOR_ARGS="script.py,arg1,arg2" cargo run
    let command = env::var("MONITOR_CMD").unwrap_or_else(|_| "ping".to_string());
    let args_str = env::var("MONITOR_ARGS").unwrap_or_else(|_| "localhost,-c,100".to_string());
    let args: Vec<&str> = args_str.split(',').collect();

    println!("[LOG COLLECTOR] Executando: {} {:?}", command, args);

    let mut child = Command::new(command)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao executar o comando");

    let stdout = child.stdout.take().expect("Falha ao capturar stdout");
    let stderr = child.stderr.take().expect("Falha ao capturar stderr");

    let store_stdout = store.clone_handle();
    let store_stderr = store.clone_handle();

    // Task para ler stdout
    let stdout_task = tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            println!("[STDOUT] {}", line);
            store_stdout.add(line.into_bytes());
        }
    });

    // Task para ler stderr
    let stderr_task = tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            println!("[STDERR] {}", line);
            store_stderr.add(line.into_bytes());
        }
    });

    // Aguarda ambas as tasks de leitura
    let _ = tokio::join!(stdout_task, stderr_task);

    // Aguarda o processo terminar
    let status = child.wait().await.expect("Processo falhou");
    println!("[LOG COLLECTOR] Processo finalizado com status: {}", status);
}

async fn start_http_server(store: Store) {
    println!("[HTTP SERVER] Iniciando servidor na porta 3000...");

    let app_state = AppState { store };

    let app = Router::new()
        .route("/", get(root))
        .route("/logs", get(get_logs))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("[HTTP SERVER] Servidor rodando em http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Log Collector API - Use GET /logs para recuperar logs"
}

async fn get_logs(State(state): State<AppState>) -> (StatusCode, Json<LogResponse>) {
    match state.store.retrieve_first() {
        Ok(batch) => {
            // Converte Vec<Vec<u8>> para Vec<String>
            let logs: Vec<String> = batch
                .iter()
                .filter_map(|bytes| String::from_utf8(bytes.clone()).ok())
                .collect();

            (
                StatusCode::OK,
                Json(LogResponse {
                    success: true,
                    logs: Some(logs),
                    error: None,
                }),
            )
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(LogResponse {
                success: false,
                logs: None,
                error: Some(e.to_string()),
            }),
        ),
    }
}
