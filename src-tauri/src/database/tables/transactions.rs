use rusqlite::Connection;

pub struct Transaction {
    transaction_id: Option<i32>,
    item_id: i32,
    cost: i32,
    notes: String,
    transaction_type: i32,
    payment_method_id: i32,
    date_time: i32,
}

impl Transaction {
    pub fn new(
        transaction_id: Option<i32>,
        item_id: i32,
        cost: i32,
        notes: String,
        transaction_type: i32,
        payment_method_id: i32,
        date_time: i32,
    ) -> Self {
        Transaction {
            transaction_id,
            item_id,
            cost,
            notes,
            transaction_type,
            payment_method_id,
            date_time,
        }
    }
    pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
        let sql = "CREATE TABLE IF NOT EXISTS Transactions (
            TransactionsId INTEGER PRIMARY KEY AUTOINCREMENT,
            ItemId INTEGER,
            Cost INTEGER NOT NULL,
            Notes TEXT,
            TransactionType INTEGER NOT NULL,
            PaymentMethodId INTEGER NOT NULL,
            DateTime INTEGER NOT NULL,
            FOREIGN KEY(ItemId) REFERENCES Item(ItemId),
            FOREIGN KEY(PaymentMethodId) REFERENCES PaymentMethod(PaymentMethodId)
        );";
        conn.execute(sql, [])?;
        Ok(())
    }
}
