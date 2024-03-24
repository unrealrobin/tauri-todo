use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::Result as IoResult};
use std::fs;

//use std::collections::BTreeMap;

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

#[tauri::command]
pub fn delete_item(key: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = read_db()?;

    map.remove(&key);

    let mut new_map = BTreeMap::new();
    for(new_key, (_old_key, value)) in map.into_iter().enumerate() {
        new_map.insert(new_key as i32, value);
    }

    write_to_db(new_map)?;

    Ok(())
}

#[tauri::command]
pub fn update_item(key: i32, text: &str, new_status: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut list = read_db()?;

    if let Some(item) = list.get_mut(&key) {
        item.text = String::from(text);
        item.is_complete = new_status;
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found in list")));
    }

    clear_db()?;
    write_to_db(list)?;

    Ok(())
}

pub fn read_db () -> Result<BTreeMap<i32, TodoItem>, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string("C:/folio/tododb.json")?;
    let todo_list: BTreeMap<i32, TodoItem> = serde_json::from_str(&data)?;

    println!("Data from db is: {:?}", todo_list);
    Ok(todo_list)
}

pub fn add_to_list(item: TodoItem) -> Result<(), Box<dyn std::error::Error>> {

    let mut list = read_db()?;

    list.insert(list.len() as i32, item );

    clear_db()?;

    write_to_db(list)?;

    Ok(())

}

pub fn write_to_db(todo_list: BTreeMap<i32, TodoItem>) -> IoResult<()> {
    let list = serde_json::to_string_pretty(&todo_list).expect("Failed to Serialize.");


    //TODO: Write this JSON to the tododb.json file
    fs::write("C:/folio/tododb.json", list)?;


    Ok(())
}



pub fn clear_db () -> Result<(), std::io::Error> {
    fs::File::create("C:/folio/tododb.json")?;
    Ok(())
}

pub fn initialize_db () -> Result<(), Box<dyn std::error::Error>> {

    let map: BTreeMap<i32, TodoItem> = BTreeMap::new();

    write_to_db(map)?;

    Ok(())

}

