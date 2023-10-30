use rusqlite::Connection;

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
        Ok(())
    }

    pub fn insert_data(data: String, conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "INSERT INTO Category(Name) VALUES(:name)";
        conn.execute(sql, &[(":name", &data)])?;
        Ok(())
    }
}
