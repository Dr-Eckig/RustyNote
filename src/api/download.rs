use gloo::utils::window;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, js_sys::Array};

pub fn download_file(content: String, filename: &str) {
        
    let blob_parts = Array::new();
    blob_parts.push(&content.into());
    let blob = web_sys::Blob::new_with_str_sequence(&blob_parts).unwrap();

    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let document = window().document().unwrap();
    let a: HtmlAnchorElement = document
        .create_element("a").unwrap()
        .dyn_into().unwrap();

    a.set_href(&url);
    a.set_download(filename);
    a.click();

    web_sys::Url::revoke_object_url(&url).unwrap();
}