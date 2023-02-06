use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, vec};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Pringle!");

    body.append_child(&val)?;

    Ok(())
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[derive(Serialize, Deserialize)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn clamp(val: u8, min: u8, max: u8) -> u8 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}

#[wasm_bindgen]
pub fn process_image(val: &JsValue) -> JsValue {
    let img: ImageData = val.into_serde().unwrap();
    let mut data = img.data;
    // invert the image,,, just as a test
    for i in (0..data.len()).step_by(4) {
        for j in (0..3).step_by(1) {
            data[i + j] = clamp(255 - data[i + j], 0, 255) as u8;
        }
    }
    return JsValue::from_serde(&data).unwrap(); 
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.]
    };

    JsValue::from_serde(&example).unwrap()
}


#[wasm_bindgen]
pub fn receive_example_from_js(val: &JsValue) {
    let example: Example = val.into_serde().unwrap();
}