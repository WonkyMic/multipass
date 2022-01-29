use crate::data::twitter;

pub mod tweets;
pub mod user;

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *twitter::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    
    headers.insert(reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN, reqwest::header::HeaderValue::from_str("http://localhost:8080").unwrap());
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    assert_eq!(headers["Access-Control-Allow-Origin"], "http://localhost:8080");

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
