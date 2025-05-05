use rusqlite::{Connection, Result, params};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("testing.db")?;
    
    // Crear tabla Gastos si no existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS gastos (
            expense_id TEXT PRIMARY KEY,
            date TEXT NOT NULL,
            amount TEXT NOT NULL,
            category TEXT NOT NULL,
            payment_method TEXT NOT NULL,
            vendor TEXT NOT NULL,
            description TEXT
        )",
        [],
    )?;
    
    // Crear tabla Ingresos si no existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ingresos (
            revenue_id TEXT PRIMARY KEY,
            revenue_date TEXT NOT NULL,
            revenue_amount TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}