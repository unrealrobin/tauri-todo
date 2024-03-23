use serde::{Deserialize, Serialize};
use std::io::Result as IoResult;
use serde_json::Result as JsonResult;
use std::fs;

//use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    text: String,
    is_complete: bool,
}

pub fn create_item(text: &str) -> TodoItem {
    let new_item = TodoItem {
        text: String::from(text),
        is_complete: false,
    };

    return new_item;
}


pub fn write_to_db(todo_item: TodoItem) -> IoResult<()> {
    let item = serde_json::to_string(&todo_item).expect("Failed to Serialize.");

    //TODO: Write this JSON to the tododb.json file
    fs::write("C:/folio/tododb.json", item)?;

    read_db();

    Ok(())
}

pub fn read_db () -> Result<TodoItem, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string("C:/folio/tododb.json")?;
    let todo_item: TodoItem = serde_json::from_str(&data)?;

    println!("Data from db is: {:?}", todo_item);
    Ok(todo_item)
}