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

    pub use web_sys::*;

    pub use crate::list_filters::*;
    pub use crate::words::*;
    pub use crate::wordle_word::WordleWord;
}

use prelude::*;
use wasm_bindgen::JsCast;

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
// Our WASM function to get input from JavaScript
#[wasm_bindgen]
pub fn get_words(
    first: &str,
    second: &str,
    third: &str,
    fourth: &str,
    fifth: &str,
    found: &str,
    incorrect: JsValue,
) -> Result<(), JsValue> {
    let guess = [
        first,
        second,
        third,
        fourth,
        fifth,
    ];

    // All of our selected letters that are incorrect responses from JS
    let incorrect_results: Vec<char> = incorrect
        .into_serde()
        .expect("Error parsing incorrect letters");

    // format letters that were contained but not correct into lower case
    let contained = String::from(found).to_lowercase();

    // filter out words that contain incorrect results
    let wordle_words = filter_incorrect_letters(incorrect_results);

    // filter to keep words that have the correct letters in the correct place
    let wordle_words: Vec<WordleWord> = filter_correct_letters(guess, wordle_words);

    // filter to keep words
    let mut results = filter_found_letters(&contained, wordle_words);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let results_element = document
        .get_element_by_id("results_value")
        .expect("Error getting results element")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Error converting to HtmlElement");

    let word_freq_results_list = document
        .get_element_by_id("freq_table")
        .expect("Error getting results element")
        .dyn_into::<web_sys::HtmlTableElement>()
        .expect("Error converting to HtmlElement");
        
    results_element.set_text_content(Some(&results.len().to_string()));

    // let mut freq_results: String = String::new();

    results.sort_by(|a, b| b.zipf_freq.partial_cmp(&a.zipf_freq).unwrap());
    for (i, x) in results.iter().enumerate() {
        if i == 50 {
            break;
        }
        // freq_results = freq_results + &(i + 1).to_string() + ". " + &x.zipf_freq.to_string() + "\t";
        // let table_data = document.create_element("td")?;
        let table_row = word_freq_results_list.insert_row()?;
        let text = &[&(i + 1).to_string(),". ",&x.zipf_freq.to_string()].concat();
        let text = &document.create_text_node(text);
        table_row.append_child(text)?;
        word_freq_results_list.append_child(&table_row)?;
    }

    // word_freq_results_list.set_text_content(Some(&freq_results));
    // log(&freq_results);

    Ok(())
}