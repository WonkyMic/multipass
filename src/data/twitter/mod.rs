use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub mod tweet;
pub mod user;

lazy_static! {
    pub static ref BEARER: &'static str = dotenv!("TWITTER_BEARER");
}

lazy_static! {
    pub static ref URL: &'static str = "https://api.twitter.com/2";
}

#[derive(Debug, Serialize)]
pub struct TwitterProfile {
    pub user: User,
    pub followers: Vec<User>,
    pub following: Vec<User>,
    pub recent: Vec<Tweet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    id: String,
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
}

