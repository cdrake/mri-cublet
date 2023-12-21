// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::JSON::stringify;
// // use serde_json;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn log_init() {
//   log("initialized");
// }

// #[wasm_bindgen]
// pub async fn load_json(url: String) -> Result<JsValue, JsValue> {
//   log("loading json");
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     // let url = format!("https://api.github.com/repos/{}/branches/master", repo);

//     let request = Request::new_with_str_and_init(&url, &opts)?;

//     // request
//     //     .headers()
//     //     .set("Accept", "application/vnd.github.v3+json")?;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     // `resp_value` is a `Response` object.
//     assert!(resp_value.is_instance_of::<Response>());
//     let resp: Response = resp_value.dyn_into().unwrap();

//     // Convert this other `Promise` into a rust `Future`.
//     let json = JsFuture::from(resp.json()?).await?;
//     // log(serde_json::to_string(&json).unwrap());
//     let json_string = stringify(&json).unwrap().as_string().unwrap();
//     log(&json_string);
//     // Send the JSON response back to JS.
//     Ok(json)
// }
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn load_json() -> Result<JsValue, JsValue> {
    log("loading json");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = "http://127.0.0.1:8080/glossary.json";
    // let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    // request
    //     .headers()
    //     .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    let json_string = stringify(&json).unwrap().as_string().unwrap();
    log(&json_string);
    // Send the JSON response back to JS.
    Ok(json)
}