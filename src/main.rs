mod words;
mod list_filters;

mod prelude {
    pub use std::io::prelude::*;
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::Read;

    pub use crate::list_filters::*;
    pub use crate::words::*;
}

use std::fs;

use prelude::*;

fn main() {
    // dbg!(words::answer_list.len());
    // dbg!(words::allowed_list.len());
    let guess = String::from("p\0\0\0\0");
    let contained = String::from("e");
    let found = get_words_letters_contained(&contained, get_words_letters_in_placed(&guess));

    println!("Possible word count {}", found.len());
    println!("words {:?}", found);
    
}

fn cleanup_input() {
    let contents = fs::read_to_string("temp").unwrap();
    let split: Vec<&str> = contents.split('\n').collect();
    for s in split {
        println!("{}", s);
        write_to_file(&["\"", &s, "\","].concat(), "out");
    }

}

fn write_to_file(line: &str, path: &str) {
    match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
        Ok(mut s) => s.write_all([line, "\n"].concat().as_bytes()).expect("Error writing to file"), 
        Err(why) => panic!("{}", why),
    }
}