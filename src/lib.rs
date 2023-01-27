use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::general_purpose};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;

//function to grayscale an image, which will be passed via WASM to the js part.
//We receive a base64 encoded string with the image
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());
    //we decode the base64 string using the general purpose engine of the base64 library
    //it returns a result object which we unwrap to a Vec<u8>
    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());
    //we use the image crate to load the image from the Vec<u8> returned by the base64 decoder
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
    //we grayscale the image via the grayscale method of the DynamicImage enum. It returns another DynamicImage enum, hence the mutable variable
    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    //we create a buffer to start the conversion process back to bytes.
    //Due to an update to the image library, from 0.24+ you HAVE to implement a seek method on the buffer.
    //We do this by wrapping it around a cursor.
    let mut buffer = Cursor::new(vec![]);
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    //we consume the cursor object, leaving only the vector to pass with the AsRef impl to the encode method
    let vec = buffer.into_inner();

    //we finally encode the resulting vector and format it with the necessary metadata to pass it back to the browser
    let encoded_image = general_purpose::STANDARD.encode(vec);
    let data_url = format! (
        "data:image/png;base64,{}",
        encoded_image
    );

    return data_url;
}