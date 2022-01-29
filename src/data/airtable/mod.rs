use lazy_static::lazy_static;

//pub mod translation;
pub mod twitter;

lazy_static! {
    pub static ref BEARER: &'static str = dotenv!("AIRTABLE_BEARER");
}

lazy_static! {
    pub static ref URL: &'static str = dotenv!("AIRTABLE_URL");
}


