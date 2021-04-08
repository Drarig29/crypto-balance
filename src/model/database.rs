use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootObject {
    pub snapshots: Vec<Snapshot>,
}

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
