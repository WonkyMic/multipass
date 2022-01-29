use crate::data;

//pub mod translation;
pub mod twitter;

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *data::airtable::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
