// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod database;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
    "Money Memo".to_string()
}

pub struct Conn {
    conn: Option<rusqlite::Connection>,
}
impl Conn {
    pub fn new(conn: Option<rusqlite::Connection>) -> Self {
        Conn {
            conn, // 使用 Some 包装 Connection
        }
    }
    pub fn get_conn(&mut self) -> &Option<rusqlite::Connection> {
        &self.conn
    }
    pub fn set_conn(&mut self, conn: rusqlite::Connection) {
        self.conn = Some(conn);
    }
}
fn main() {
    // database::connection();
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
    tauri::Builder::default()
        .setup(|app| {
            Ok({
                let new_conn: std::sync::Mutex<Conn> = std::sync::Mutex::new(Conn::new(None));
                app.manage(new_conn);
            })
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            database::create_connection,
            database::is_database_exist,
            database::create_new_tables,
            database::insert_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
