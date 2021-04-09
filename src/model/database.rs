use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub time: DateTime,
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
    pub time: DateTime,
    pub price: f32,
}