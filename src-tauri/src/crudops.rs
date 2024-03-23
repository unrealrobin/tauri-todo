use crate::setupfile::setup_dir;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Result as IoResult};
use std::fs;

//use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    text: String,
    is_complete: bool,
}

#[tauri::command]
pub fn create_item(text: &str)  {
    let new_item = TodoItem {
        text: String::from(text),
        is_complete: false,
    };

    add_to_list(new_item).expect("failed to add to list");
}

pub fn add_to_list(item: TodoItem) -> Result<(), Box<dyn std::error::Error>> {

    let mut list = read_db()?;

    list.insert(list.len() as i32, item );

    clear_db()?;

    write_to_db(list)?;

    Ok(())

}

pub fn write_to_db(todo_list: HashMap<i32, TodoItem>) -> IoResult<()> {
    let list = serde_json::to_string_pretty(&todo_list).expect("Failed to Serialize.");


    //TODO: Write this JSON to the tododb.json file
    fs::write("C:/folio/tododb.json", list)?;


    Ok(())
}

pub fn read_db () -> Result<HashMap<i32, TodoItem>, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string("C:/folio/tododb.json")?;
    let todo_list: HashMap<i32, TodoItem> = serde_json::from_str(&data)?;

    println!("Data from db is: {:?}", todo_list);
    Ok(todo_list)
}

pub fn clear_db () -> Result<(), std::io::Error> {
    fs::File::create("C:/folio/tododb.json")?;
    Ok(())
}

pub fn initialize_db () -> Result<(), Box<dyn std::error::Error>> {
    setup_dir()?;

    let map: HashMap<i32, TodoItem> = HashMap::new();

    write_to_db(map)?;

    Ok(())

}

#[tauri::command]
pub fn delete_item(key: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = read_db()?;

    map.remove_entry(&key);

    write_to_db(map)?;

    Ok(())
}