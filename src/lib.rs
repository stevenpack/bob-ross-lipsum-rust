#![feature(test)]
extern crate rand;
extern crate test;
extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
mod phrases;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, bob-ross-lorem-ipsum-rust!");
}

#[wasm_bindgen]
pub fn get_phrase_text(phrase_cnt: usize, new_line: usize) -> String {
    phrases::get_phrase_text(phrase_cnt, new_line)
}