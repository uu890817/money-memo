use rusqlite::Connection;
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
