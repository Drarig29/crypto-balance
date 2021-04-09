use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub time: i64,
    pub total_asset_of_btc: f32,
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub amount: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyHistory {
    pub asset: String,
    pub history: Vec<HistoricPrice>,
}

#[derive(Serialize, Deserialize)]
pub struct HistoricPrice {
    pub time: i64,
    pub price: f32,
}