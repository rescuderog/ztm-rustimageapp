use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::general_purpose};

//function to grayscale an image, which will be passed via WASM to the js part.
//We receive a base64 encoded string with the image
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale called".into());
    //we decode the base64 string using the general purpose engine of the base64 library
    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file);
    log(&"Image decoded".into());
}