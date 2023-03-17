use crate::schema::events;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub option_token: String,
    pub capital_transfered: String,
    pub option_tokens_minted: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub event_index: i64,
    pub from_address: String,
    pub timestamp: i64,
    pub action: String,
    pub caller: String,
    pub option_token: String,
    pub capital_transfered: String,
    pub option_tokens_minted: String,
}
