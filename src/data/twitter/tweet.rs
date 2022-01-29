use crate::data::twitter;
use serde::Deserialize;

#[derive(Debug, Deserialize)] 
pub struct Response { 
    pub data: Vec<twitter::Tweet>,
    meta: Metadata
}

#[derive(Debug, Deserialize)]
struct Metadata {
    oldest_id: String,
    newest_id: String,
    result_count: u8,
    next_token: String
}
