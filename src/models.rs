// models.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gastos {
    pub expense_id: String,
    pub date: String,
    pub amount: String,
    pub category: String,
    pub payment_method: String,
    pub vendor: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingresos {
    pub revenue_id: String,
    pub revenue_date: String,
    pub revenue_amount: String,
}

