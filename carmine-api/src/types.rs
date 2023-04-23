use carmine_api_core::types::{Event, TradeHistory};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct DataResponse<T> {
    pub status: String,
    pub data: T,
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

#[derive(Serialize, Debug)]
pub struct AllTradeHistoryResponse<'a> {
    pub status: String,
    pub data: Vec<&'a TradeHistory>,
    pub length: usize,
}

#[derive(Serialize)]
pub struct EventsResponse {
    pub status: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub address: Option<String>,
}
