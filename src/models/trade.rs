use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<i64>,
    pub master_account_id: i64,
    pub ticket: i64,
    pub master_ticket: i64,  // Add this line
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub open_price: f64,
    pub open_time: String,
    pub close_price: Option<f64>,
    pub close_time: Option<String>,
    pub profit: Option<f64>,
    pub status: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
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
    pub server_id: i64,  // Add this field
    pub symbol: String,
    pub close_price: f64,
    pub close_time: String,
    pub profit: f64,
}

#[derive(Debug, Deserialize)]
pub struct TPSLUpdate {
    pub master_account_id: i64,
    pub server_id: i64,
    pub take_profit: f64,
    pub stop_loss: f64,
}