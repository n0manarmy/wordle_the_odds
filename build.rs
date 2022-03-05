use serde::{Deserialize, Serialize};
// use std::env;
use std::path::Path;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct WordList {
    answers: Vec<String>,
    filter: Vec<String>
}

fn build_list(word_list: Vec<String>) -> String {
    let mut list: String = String::new();
    for w in word_list {
        list = list + &format!("\"{}\",\n", w);
    }
    list
}

fn main() {
    let file = std::fs::File::open("words.json").unwrap();
    let file = std::io::BufReader::new(file);
    let word_lists: WordList = serde_json::from_reader(file).expect("error parsing json file");
    
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = "./src";
    let dest_path = Path::new(&out_dir).join("words.rs");
    let output: String = format!(
    "pub static test_answer_list: [&str; 5] = [
        \"aback\",
        \"abase\",
        \"abate\",
        \"abbey\",
        \"abbot\"
    ];
    
    pub static test_filter_list: [&str; 5] = [
        \"aahed\",
        \"aalii\",
        \"aargh\",
        \"aarti\",
        \"abaca\"
    ];
    
    pub static answer_list: [&str; {}] = [{}];", word_lists.answers.len(), build_list(word_lists.answers));
    
    fs::write(
        &dest_path, output   
    ).unwrap();
    println!("carg:rerun-if-changed=build.rs");
}

// fn cleanup_input() {
//     let contents = fs::read_to_string("temp").unwrap();
//     let split: Vec<&str> = contents.split('\n').collect();
//     for s in split {
//         println!("{}", s);
//         write_to_file(&["\"", &s, "\","].concat(), "out");
//     }

// }

// fn write_to_file(line: &str, path: &str) {
//     match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
//         Ok(mut s) => s.write_all([line, "\n"].concat().as_bytes()).expect("Error writing to file"), 
//         Err(why) => panic!("{}", why),
//     }
// }