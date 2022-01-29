use crate::data::twitter;
use serde::Deserialize;

// User Lookup
#[derive(Debug, Deserialize)] pub struct Response { pub data: twitter::User }

// Followers Lookup
#[derive(Debug, Deserialize)] pub struct FollowResponse { pub data: Vec<twitter::User> } 
