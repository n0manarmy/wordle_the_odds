mod list_filters;
mod words;
mod wordle_word;
mod prelude {
    extern crate serde_json;
    extern crate wasm_bindgen;

    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::io::Read;
    pub use std::path::Path;

    pub use serde::{Deserialize, Serialize};
    pub use wasm_bindgen::prelude::*;

    pub use crate::list_filters::*;
    pub use crate::words::*;
    pub use crate::wordle_word::WordleWord;
}

use prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_words(
    first: &str,
    second: &str,
    third: &str,
    fourth: &str,
    fifth: &str,
    found: &str,
    incorrect: JsValue,
) -> String {
    let guess = [
        first.chars().next().unwrap(),
        second.chars().next().unwrap(),
        third.chars().next().unwrap(),
        fourth.chars().next().unwrap(),
        fifth.chars().next().unwrap(),
    ];

    let incorrect_results: Vec<char> = incorrect
        .into_serde()
        .expect("Error parsing incorrect letters");

    let contained = String::from(found).to_lowercase();
    let wordle_words = filter_incorrect_letters(incorrect_results);
    let wordle_words: Vec<WordleWord> = filter_correct_letters(guess, wordle_words);
    let results = filter_found_letters(&contained, wordle_words);

    results.len().to_string()
}