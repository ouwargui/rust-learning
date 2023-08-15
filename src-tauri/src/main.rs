// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn load_asynchronous() -> Result<String, String> {
    let res = reqwest::get("https://httpbin.org/ip").await;
    let body = res.unwrap().text().await.unwrap();

    Ok(format!("{}", body))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tauri::command]
async fn fetch_todos() -> Result<Vec<Todo>, String> {
    let res = reqwest::get("https://jsonplaceholder.typicode.com/todos").await;
    let body = res.unwrap().text().await.unwrap();

    let todos: Vec<Todo> = serde_json::from_str(&body).unwrap();

    Ok(todos)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_asynchronous,
            fetch_todos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
