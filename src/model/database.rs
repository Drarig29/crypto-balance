use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;

/* Collection: snapshots */

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

/* Collection: history */

#[derive(Serialize, Deserialize)]
pub struct CurrencyHistory {
    pub time: DateTime,
    pub asset: String,
    pub price: f32,
}

/* Aggregation results */

#[derive(Serialize, Deserialize)]
pub struct ComputedSnapshot {
    pub time: DateTime,
    pub total_asset_of_btc: ComputedBalance,
    pub balances: Vec<ComputedBalance>,
}

#[derive(Serialize, Deserialize)]
pub struct ComputedBalance {
    pub asset: String,
    pub amount: f32,
    pub price: Option<f32>,
    pub value: Option<f32>,
}