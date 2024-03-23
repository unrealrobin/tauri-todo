// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setupfile;
mod crudops;
use setupfile::setup_dir;
use crudops::{create_item, delete_item, initialize_db, update_item};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//
 #[tauri::command]
 fn greet(name: &str) -> String {
     format!("Hello, {}! You've been greeted from Rust!", name)
 }

fn main() {

    match setup_dir() {
        Ok(()) => {

            initialize_db().expect("failed to initialize");
            create_item("first todo");
            create_item("second todo");
            create_item("third todo");
            delete_item(1).expect("Could not Find Todo.");
            update_item(0, "new first todo", true).expect("Failed to Update");

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
