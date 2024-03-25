// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setupfile;
use setupfile::setup_dir;
mod crudops;



fn main() {

    match setup_dir() {
        Ok(()) => {
            tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![crudops::read_db, crudops::create_item, crudops::delete_item, crudops::update_item])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        }
        Err(e) => {
            eprintln!("Failed to set up Directory. {}", e);
            std::process::exit(1);
        }
    }

    
}
