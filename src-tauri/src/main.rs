// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setupfile;
use setupfile::setup_dir;
mod crudops;
use crudops::read_db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//
 #[tauri::command]
 fn greet(name: &str) -> String {
     format!("Hello, {}! You've been greeted from Rust!", name)
 }

fn main() {

    match setup_dir() {
        Ok(()) => {
            tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![crudops::read_db])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        }
        Err(e) => {
            eprintln!("Failed to set up Directory. {}", e);
            std::process::exit(1);
        }
    }

    
}
