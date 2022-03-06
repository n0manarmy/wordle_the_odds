mod words;
mod list_filters;
mod prelude {
    extern crate serde_json;
    extern crate wasm_bindgen;
    
    pub use std::io::prelude::*;
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::Read;

    pub use wasm_bindgen::prelude::*;

    pub use crate::list_filters::*;
    pub use crate::words::*;
}

use prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_words(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, found: &str, incorrect: JsValue) -> String {
    // log("get_words enter");
    // log(&["first ",  first].concat());
    // log(&["second ",  second].concat());
    // log(&["third ",   third].concat());
    // log(&["fourth ",  fourth].concat());
    // log(&["fifth ",  fifth].concat());
    // log(&["found ",  found].concat());

    let guess = [
        first.chars().next().unwrap(), 
        second.chars().next().unwrap(), 
        third.chars().next().unwrap(), 
        fourth.chars().next().unwrap(), 
        fifth.chars().next().unwrap()
        ];
    
    let incorrect_results: Vec<char> = incorrect.into_serde().expect("Error parsing incorrect letters");

    // let inc_change: Vec<char> = incorrect_results.iter().map(|i| i.chars().next().unwrap()).collect();
    // let inc: String = incorrect_results.iter().map(|i| i.to_lowercase().to_string()).collect();
    // log(&["found incorrect", &inc].concat());

    // let guess = String::from([first, second, third, fourth, fifth].concat()).to_lowercase();
    let contained = String::from(found).to_lowercase();
    let words = filter_incorrect_letters(incorrect_results);
    let words_found: Vec<&str> = filter_correct_letters(guess, words);
    let results = filter_found_letters(&contained, words_found);

    // log("get_words done");

    results.len().to_string()

    // log("Possible word count {}", found.len());
    // log("words {:?}", found);
}