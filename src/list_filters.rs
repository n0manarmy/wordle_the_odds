use crate::prelude::*;

pub fn get_words_letters_in_placed(letters: &str) -> Vec<String> {

    let mut found: Vec<String> = Vec::new();
    for line in answer_list {
        let mut exists = false;
        for (i, c) in line.char_indices() {
            if letters.as_bytes()[i] as char == '\0' {
                continue;
            }
            if letters.as_bytes()[i] as char == c {
                exists = true;
                
            } else {
                exists = false;
                break;
            }
        }
        if exists {
            found.push(String::from(line));
        }
    }

    found
}

pub fn get_words_letters_contained(letters: &str, words_found: Vec<String>) -> Vec<String> {
    let mut words_left: Vec<String> = Vec::new();
    for word in words_found {
        let mut exists = false;
        for c in letters.chars() {
            if word.contains(c) {
                exists = true;
            } else {
                exists = false;
                break;
            }
        }
        if exists {
            words_left.push(String::from(word));
        }
    }

    words_left
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_get_words_letters_in_place() {
        let letters: String = String::from("a\0\0\0\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 5);
        let letters: String = String::from("a\0\0\0\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 5);
        let letters: String = String::from("ab\0\0\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 5);
        let letters: String = String::from("aba\0\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 3);
        let letters: String = String::from("aback");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 1);
        assert_eq!(letters, get_words_letters_in_placed(&letters)[0]);
        let letters: String = String::from("a\0\0e\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 1);
        let letters: String = String::from("\0\0\0\0\0");
        assert_eq!(get_words_letters_in_placed(&letters).len(), 0);
    }

    #[test]
    pub fn test_get_words_letters_contained() {
        let letters: String = String::from("aba\0\0");
        let words_found  = get_words_letters_in_placed(&letters);
        let contains = String::from("et");
        assert_eq!(get_words_letters_contained(&contains, words_found), vec![String::from("abate")]);
    }
}