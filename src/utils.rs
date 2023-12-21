use js_sys::{Object, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamDefaultReader, Request, RequestInit, RequestMode, Response};
use futures::io::Error;
use futures::io::ErrorKind::NotFound;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
extern "C" {
    /// Log a string value to the console.
    #[allow(unused)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn fetch_json_js(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

#[wasm_bindgen]
pub async fn fetch_binary_js(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    log(&format!("Response status code: {}", resp.status()));
    if resp.status() != 200 {
        return Err(JsValue::FALSE);
    }

    let reader_value = resp.body().unwrap().get_reader();
    let reader: ReadableStreamDefaultReader = reader_value.dyn_into().unwrap();
    let result_value = JsFuture::from(reader.read()).await?;
    let result: Object = result_value.dyn_into().unwrap();
    let chunk_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
    let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
    Ok(chunk_array.into())
}

pub async fn fetch_binary(url: String) -> Result<Vec<u8>, Error> {
  let _uint_array = match fetch_binary_js(url).await {    
    Ok(_uint_array) => return Ok(Uint8Array::from(_uint_array).to_vec()),
    Err(_e) => return Err(NotFound.into())
  };  
}

