use rusqlite::Connection;
pub struct AppConfig {
    settings: String,
    config: String,
}

impl AppConfig {
    pub fn new(settings: String, config: String) -> Self {
        AppConfig { settings, config }
    }

    pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS AppConfig (
            Setting TEXT,
            Config TEXT
        );";
        conn.execute(sql, [])?;
        Ok(())
    }
}
