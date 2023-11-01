use rusqlite::Connection;

use crate::database::remove_quotes;

pub struct PaymentMethod {
    payment_method_id: Option<i32>,
    name: String,
}

impl PaymentMethod {
    pub fn new(payment_method_id: Option<i32>, name: String) -> Self {
        PaymentMethod {
            payment_method_id,
            name,
        }
    }

    pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS PaymentMethod (
            PaymentMethodId INTEGER PRIMARY KEY AUTOINCREMENT,
            Name TEXT NOT NULL
        );";
        conn.execute(sql, [])?;
        Ok(())
    }

    pub fn insert_data(data: String, conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "INSERT INTO PaymentMethod(Name) VALUES(:Name)";
        let parsed_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);
        match parsed_data {
            Ok(json) => {
                conn.execute(sql, &[(":Name", remove_quotes(&json["Name"].to_string()))])?;
            }
            Err(_) => todo!(),
        }
        Ok(())
    }
}
