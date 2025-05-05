mod models;
mod db;
mod operations;

use operations::*;
use models::{Gastos, Ingresos};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ejemplo con Gastos
    let nuevo_gasto = Gastos {
        expense_id: Uuid::new_v4().to_string(),
        date: "2023-05-01".to_string(),
        amount: "150.50".to_string(),
        category: "Comida".to_string(),
        payment_method: "Tarjeta".to_string(),
        vendor: "Supermercado".to_string(),
        description: "Compra semanal".to_string(),
    };
    
    // create_gasto(nuevo_gasto)?;
    delete_gasto("dbadb464-d76b-4f22-91cd-94b38eb7143c")?;
    
    let gastos = read_gastos()?;
    // println!("Gastos: {:?}", gastos);
    
    // Ejemplo con Ingresos
    let nuevo_ingreso = Ingresos {
        revenue_id: Uuid::new_v4().to_string(),
        revenue_date: "2023-05-01".to_string(),
        revenue_amount: "2000.00".to_string(),
    };
    
    //create_ingreso(nuevo_ingreso)?;
    
    let ingresos = read_ingresos()?;
    // println!("Ingresos: {:?}", ingresos);
    
    Ok(())
}