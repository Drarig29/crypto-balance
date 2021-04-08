use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootObject {
    pub code: i64,
    pub msg: String,
    #[serde(rename = "snapshotVos")]
    pub snapshots: Vec<Snapshot>,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "totalAssetOfBtc")]
    pub total_asset_of_btc: String,
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}
