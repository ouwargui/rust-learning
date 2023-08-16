// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: i32, title: &str, completed: bool) -> Self {
        Self {
            id,
            title: title.to_string(),
            completed,
        }
    }
}

struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Self {
            todos: vec![Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                completed: false,
            }],
        }
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn get(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn remove(&mut self, todo_id: i32) {
        self.todos.retain(|todo| todo.id != todo_id);
    }
}

struct State {
    todos: Mutex<Todos>,
}

#[tauri::command]
async fn fetch_todos(state: tauri::State<'_, State>) -> Result<Vec<Todo>, String> {
    let todos_unwrapped = state.todos.lock().unwrap();
    let todos = todos_unwrapped.get();
    Ok(todos.to_vec())
}

#[tauri::command]
async fn add_todo(todo: Todo, state: tauri::State<'_, State>) -> Result<(), String> {
    let new_todo = Todo::new(todo.id, &todo.title, todo.completed);
    state.todos.lock().unwrap().add(new_todo);
    Ok(())
}

#[tauri::command]
async fn remove_todo(id: i32, state: tauri::State<'_, State>) -> Result<(), String> {
    state.todos.lock().unwrap().remove(id);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(State {
            todos: Mutex::new(Todos::new()),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            load_asynchronous,
            fetch_todos,
            add_todo,
            remove_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
