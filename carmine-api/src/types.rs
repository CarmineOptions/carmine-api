use carmine_api_db::models::{Event, TradeHistory};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct AllNonExpired<'a> {
    pub status: String,
    pub data: &'a Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TradeHistoryResponse<'a> {
    pub status: String,
    pub data: Vec<&'a TradeHistory>,
}

#[derive(Serialize)]
pub struct EventsResponse {
    pub status: String,
    pub events: Vec<Event>,
}

pub struct AppState {
    pub all_non_expired: Vec<String>,
    pub trade_history: Vec<TradeHistory>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub address: Option<String>,
}
