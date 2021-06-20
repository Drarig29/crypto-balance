use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/* Collection: snapshots */

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub time: DateTime<Utc>,
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
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub time: DateTime<Utc>,
    pub asset: String,
    pub price: f32,
}

/* Aggregation results */

#[derive(Serialize, Deserialize)]
pub struct ComputedSnapshot {
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub time: DateTime<Utc>,
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
