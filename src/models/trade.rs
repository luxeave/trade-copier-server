use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<i64>,
    pub master_account_id: i64,
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub open_price: f64,
    pub open_time: String,
    pub close_price: Option<f64>,
    pub close_time: Option<String>,
    pub profit: Option<f64>,
    pub status: String,  // Add this line
}

#[derive(Debug, Deserialize)]
pub struct SlaveInfo {
    pub slave_account_id: i64,
    pub master_account_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeClosure {
    pub master_account_id: i64,
    pub ticket: i64,
    pub symbol: String,
    pub close_price: f64,
    pub close_time: String,
    pub profit: f64,
}