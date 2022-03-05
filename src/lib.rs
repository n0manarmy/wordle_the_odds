mod words;
mod list_filters;

mod prelude {
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
pub fn get_words(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, found: &str) -> String {
    // log("get_words enter");
    // log(&["first ",  first].concat());
    // log(&["second ",  second].concat());
    // log(&["third ",   third].concat());
    // log(&["fourth ",  fourth].concat());
    // log(&["fifth ",  fifth].concat());
    // log(&["found ",  found].concat());

    let guess = String::from([first, second, third, fourth, fifth].concat()).to_lowercase();
    let contained = String::from(found).to_lowercase();
    let results = get_words_letters_contained(&contained, get_words_letters_in_placed(&guess));

    // log("get_words done");

    results.len().to_string()

    // log("Possible word count {}", found.len());
    // log("words {:?}", found);
}