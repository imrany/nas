use web_sys::{
    window,
    Request, 
    RequestInit, 
    RequestMode, 
    Response,
};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use leptos::*;


pub fn open_dialog(dialog_id:&str){
    let window=window().unwrap();
    let dialog_bg=window.document().unwrap().get_element_by_id(dialog_id).unwrap();
    dialog_bg.class_list().add_1("ease-in-out").unwrap();
    dialog_bg.class_list().add_1("block").unwrap();
    dialog_bg.class_list().add_1("duration-1000").unwrap();
    dialog_bg.class_list().add_1("delay-2000").unwrap(); 
}

pub async fn fetch_data(url:&str) ->Result<JsValue, JsValue> {
    let window=window().expect("Failed to get Window");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request
        .headers()
        .set("content-type", "application/json").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    if let Ok(resp) =  resp_value {
        let resp:Response=resp.into();
        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        // Send the JSON response back to JS.
        Ok(json)
    }else {
        Err(JsValue::from_str("Failed to fetch data"))
    }
}

pub async fn open(url:&str, path:&str) ->Result<JsValue, JsValue> {
    let window=window().expect("Failed to get Window");
     // Create a simple JSON payload
     let payload = serde_json::json!({
        "root": path
    });

    // Convert the payload to a string
    let payload_str = serde_json::to_string(&payload).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&payload_str)));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request
        .headers()
        .set("content-type", "application/json").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    if let Ok(resp) =  resp_value {
        let resp:Response=resp.into();
        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        // Send the JSON response back to JS.
        Ok(json)
    }else {
        Err(JsValue::from_str("Failed to fetch data"))
    }
}