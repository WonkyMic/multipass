use crate::data::twitter;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ Request, RequestInit, RequestMode };

pub mod tweets;
pub mod user;

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *twitter::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    
    headers.insert(reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN, reqwest::header::HeaderValue::from_str("http://localhost:8080").unwrap());
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    assert_eq!(headers["Access-Control-Allow-Origin"], "*");

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}


fn build_request(method: &str, url: &str) -> Result<Request, ()> {    
    let bearer = format!("Bearer {}", *twitter::BEARER);

    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::NoCors);
    let request = Request::new_with_str_and_init(url, &opts).expect("Failed to generate Twitter Request");
    request.headers().set("Authorization", &bearer).unwrap();
    request.headers().set("Access-Control-Allow-Origin", "http://localhost:8080/").unwrap();

    Ok(request)
}

async fn send(req: &Request) -> Result<JsValue, ()>{
    let window = web_sys::window().unwrap();
    let resp = JsFuture::from(window.fetch_with_request(&req))
            .await.unwrap();

    Ok(resp)
}

async fn get(url: &str) -> Result<JsValue, ()> {
    let req = build_request("GET", url).unwrap();
    let resp = send(&req).await.unwrap();

    Ok(resp)
}
