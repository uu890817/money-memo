use rusqlite::Connection;

use crate::database::remove_quotes;

#[derive(serde::Serialize, Debug)]
pub struct Category {
    category_id: Option<i32>,
    name: String,
}

impl Category {
    pub fn new(category_id: Option<i32>, name: String) -> Self {
        Category { category_id, name }
    }

    pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS Category (
            CategoryId INTEGER PRIMARY KEY AUTOINCREMENT,
            Name TEXT NOT NULL
        );";
        conn.execute(sql, [])?;
        Self::insert_data("{\"Name\":\"無交易類別\"}".to_string(), &conn);
        Ok(())
    }

    pub fn insert_data(data: String, conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "INSERT INTO Category(Name) VALUES(:Name)";
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
        let sql = "SELECT CategoryId, Name FROM Category";
        let mut stmt = conn.prepare(sql)?;
        let stmt_iter = stmt.query_map([], |row| {
            Ok(Category {
                category_id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut vec: Vec<Category> = Vec::new();
        for i in stmt_iter {
            match i {
                Ok(categories) => vec.push(categories),
                Err(e) => return Err(e),
            }
        }

        let json: String = serde_json::to_string(&vec).unwrap();
        Ok(json)
    }
}
