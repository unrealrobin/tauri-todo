// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setupfile;
mod crudops;
use setupfile::setup_dir;
use crudops::{create_item, write_to_db};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//
 #[tauri::command]
 fn greet(name: &str) -> String {
     format!("Hello, {}! You've been greeted from Rust!", name)
 }

fn main() {

    match setup_dir() {
        Ok(()) => {

            let some_todo = create_item("adding some todo.");

            write_to_db(some_todo).expect("Failed to Write to db");



            tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        }
        Err(e) => {
            eprintln!("Failed to set up Directory. {}", e);
            std::process::exit(1);
        }
    }

    
}
