use rusqlite::Connection;

use crate::database::remove_quotes;

#[derive(Debug, serde::Serialize)]
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
    pub fn select_all(conn: &Connection) -> Result<String, rusqlite::Error> {
        let sql = "SELECT PaymentMethodId, Name FROM PaymentMethod";
        let mut stmt = conn.prepare(sql)?;
        let stmt_iter = stmt.query_map([], |row| {
            Ok(PaymentMethod {
                payment_method_id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut vec: Vec<PaymentMethod> = Vec::new();

        for i in stmt_iter {
            match i {
                Ok(category_item) => vec.push(category_item),
                Err(e) => return Err(e),
            }
        }

        let json: String = serde_json::to_string(&vec).unwrap();
        Ok(json)
    }
}
