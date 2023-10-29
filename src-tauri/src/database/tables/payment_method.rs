use rusqlite::Connection;

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
}
