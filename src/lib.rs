#[macro_use]
extern crate dotenv_codegen;
extern crate js_sys;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen_futures;

mod clients;
mod data;
mod services;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!["Hello, {}!", name]);
}

#[wasm_bindgen]
pub async fn get_user_profile() -> Result<JsValue, JsValue> {
    let app = services::create("ThisWeekInRust", "twitter")
        .await.expect("Error creating Twitter Profile.");
    let js_value = &wasm_bindgen::JsValue::from_str(app.as_str());
    let promise = js_sys::Promise::resolve(js_value);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}
