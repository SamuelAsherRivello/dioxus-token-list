use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<u32>,
    pub total_volume: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    #[serde(default)]
    pub sparkline_in_7d: Option<TokenSparkline>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TokenSparkline {
    #[serde(default)]
    pub price: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TokenSource {
    BrowserSnapshot,
    Database,
    Online,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TokenLoadResult {
    pub tokens: Vec<Token>,
    pub source: TokenSource,
    pub online_last_updated_at: Option<DateTime<Utc>>,
    pub db_last_loaded_at: Option<DateTime<Utc>>,
}
