use rusqlite::Connection;

pub struct Item {
    item_id: Option<i32>,
    category_id: i32,
    name: String,
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
}
