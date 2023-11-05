use rusqlite::Connection;

use crate::database::remove_quotes;

#[derive(serde::Serialize, Debug)]
pub struct Item {
    item_id: Option<i32>,
    category_id: i32,
    name: String,
}
#[derive(serde::Serialize, Debug)]
pub struct CategoryItem {
    item_id: i32,
    item_name: String,
    category_id: i32,
    category_name: String,
}

impl Item {
    pub fn new(item_id: Option<i32>, category_id: i32, name: String) -> Self {
        Item {
            item_id,
            category_id,
            name,
        }
    }

    pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS Item (
            ItemId INTEGER PRIMARY KEY AUTOINCREMENT,
            CategoryId INTEGER NOT NULL,
            Name TEXT NOT NULL,
            FOREIGN KEY(CategoryId) REFERENCES Category(CategoryId)
        );";
        conn.execute(sql, [])?;
        Ok(())
    }

    pub fn insert_data(data: String, conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "INSERT INTO Item(CategoryId, Name ) VALUES(:CategoryId, :Name)";
        println!("{}", &data);

        let parsed_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);
        println!("{:?}", &parsed_data);

        match parsed_data {
            Ok(json) => {
                conn.execute(
                    sql,
                    &[
                        (
                            ":CategoryId",
                            remove_quotes(&json["CategoryId"].to_string()),
                        ),
                        (":Name", remove_quotes(&json["Name"].to_string())),
                    ],
                )?;
            }
            Err(_) => todo!(),
        }

        Ok(())
    }

    pub fn select_all(conn: &Connection) -> Result<String, rusqlite::Error> {
        let sql = "SELECT i.ItemId, i.Name, c.CategoryId, c.Name FROM Item 'i' JOIN Category 'c' ON i.CategoryId = c.CategoryId";
        let mut stmt = conn.prepare(sql)?;
        let stmt_iter = stmt.query_map([], |row| {
            Ok(CategoryItem {
                item_id: row.get(0)?,
                item_name: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
            })
        })?;

        let mut vec: Vec<CategoryItem> = Vec::new();

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
