use wasm_bindgen::prelude::*;
use chrono::{Timelike, Utc};

#[wasm_bindgen]
pub fn greet() -> String {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();

    format!(
        "Hello, world at {:02}:{:02}:{:02} {} UTC!",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    ).into()
}