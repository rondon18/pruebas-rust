use rusqlite::{params, Result};
use crate::models::{Gastos, Ingresos};
use crate::db::init_db;

// Operaciones para Gastos
pub fn create_gasto(gasto: Gastos) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO gastos (expense_id, date, amount, category, payment_method, vendor, description) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            gasto.expense_id,
            gasto.date,
            gasto.amount,
            gasto.category,
            gasto.payment_method,
            gasto.vendor,
            gasto.description
        ],
    )?;
    Ok(())
}

pub fn read_gastos() -> Result<Vec<Gastos>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT * FROM gastos")?;
    let gastos_iter = stmt.query_map([], |row| {
        Ok(Gastos {
            expense_id: row.get(0)?,
            date: row.get(1)?,
            amount: row.get(2)?,
            category: row.get(3)?,
            payment_method: row.get(4)?,
            vendor: row.get(5)?,
            description: row.get(6)?,
        })
    })?;
    
    let mut gastos = Vec::new();
    for gasto in gastos_iter {
        gastos.push(gasto?);
    }
    Ok(gastos)
}

pub fn update_gasto(gasto: Gastos) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "UPDATE gastos SET 
            date = ?2, 
            amount = ?3, 
            category = ?4, 
            payment_method = ?5, 
            vendor = ?6, 
            description = ?7 
         WHERE expense_id = ?1",
        params![
            gasto.expense_id,
            gasto.date,
            gasto.amount,
            gasto.category,
            gasto.payment_method,
            gasto.vendor,
            gasto.description
        ],
    )?;
    Ok(())
}

pub fn delete_gasto(expense_id: &str) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "DELETE FROM gastos WHERE expense_id = ?1",
        params![expense_id],
    )?;
    Ok(())
}

// Operaciones para Ingresos
pub fn create_ingreso(ingreso: Ingresos) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO ingresos (revenue_id, revenue_date, revenue_amount) 
         VALUES (?1, ?2, ?3)",
        params![
            ingreso.revenue_id,
            ingreso.revenue_date,
            ingreso.revenue_amount
        ],
    )?;
    Ok(())
}

pub fn read_ingresos() -> Result<Vec<Ingresos>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT * FROM ingresos")?;
    let ingresos_iter = stmt.query_map([], |row| {
        Ok(Ingresos {
            revenue_id: row.get(0)?,
            revenue_date: row.get(1)?,
            revenue_amount: row.get(2)?,
        })
    })?;
    
    let mut ingresos = Vec::new();
    for ingreso in ingresos_iter {
        ingresos.push(ingreso?);
    }
    Ok(ingresos)
}

pub fn update_ingreso(ingreso: Ingresos) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "UPDATE ingresos SET 
            revenue_date = ?2, 
            revenue_amount = ?3 
         WHERE revenue_id = ?1",
        params![
            ingreso.revenue_id,
            ingreso.revenue_date,
            ingreso.revenue_amount
        ],
    )?;
    Ok(())
}

pub fn delete_ingreso(revenue_id: &str) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "DELETE FROM ingresos WHERE revenue_id = ?1",
        params![revenue_id],
    )?;
    Ok(())
}