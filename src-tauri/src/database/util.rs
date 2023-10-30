use rusqlite::Connection;
use serde_json::{error::Category, json};
use std::path::Path;

use crate::Conn;

use super::tables;

#[tauri::command]
pub fn is_database_exist() -> bool {
    Path::new("Hello.db").exists()
}

#[tauri::command]
pub fn create_connection(tauri_connection: tauri::State<std::sync::Mutex<Conn>>) -> bool {
    let new_connection: Result<Connection, rusqlite::Error> = Connection::open("Hello.db");
    let mut tauri_lock_connection = tauri_connection.lock().unwrap();
    match new_connection {
        Ok(new_conn) => {
            println!("Connection Success");
            tauri_lock_connection.set_conn(new_conn);
            true
        }
        Err(e) => {
            println!("Connection Fail: {}", e);
            false
        }
    }
}

#[tauri::command]
pub fn create_new_tables(tauri_connection: tauri::State<std::sync::Mutex<Conn>>) -> bool {
    let mut tauri_lock_connection = tauri_connection.lock().unwrap();
    let connection = tauri_lock_connection.get_conn();
    match connection {
        Some(conn) => {
            tables::AppConfig::create_table(&conn);
            tables::Category::create_table(&conn);
            tables::Item::create_table(&conn);
            tables::PaymentMethod::create_table(&conn);
            tables::Transaction::create_table(&conn);
            println!("Tables Create Success");
            true
        }
        None => {
            println!("Tables Create Fail");
            false
        }
    }
}

fn remove_quotes(input: &str) -> &str {
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

#[tauri::command]
pub fn insert_data(
    to: String,
    data: String,
    tauri_connection: tauri::State<std::sync::Mutex<Conn>>,
) -> bool {
    let mut tauri_lock_connection = tauri_connection.lock().unwrap();
    let connection = tauri_lock_connection.get_conn();
    match connection {
        Some(conn) => match to.as_str() {
            "category" => {
                println!("{}", &data);
                let parsed_data: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(&data);
                match parsed_data {
                    Ok(json) => {
                        println!("{:?}", &json);
                        print!("{}", (&json["name"]).to_string());
                        tables::Category::insert_data(
                            remove_quotes((&json["name"]).to_string().as_str()).to_string(),
                            &conn,
                        );
                    }
                    Err(e) => println!("{}", e),
                }
                true
            }
            _ => false,
        },
        None => return false,
    }
}
