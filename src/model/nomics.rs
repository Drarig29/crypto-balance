use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sparkline {
    pub currency: String,
    pub timestamps: Vec<String>,
    pub prices: Vec<String>,
}