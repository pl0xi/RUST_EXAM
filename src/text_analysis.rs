use std::io;
use std::collections::HashMap;
pub fn count_words(contents: String) -> Option<usize> {
    // Split the String into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    // Count the words
    if(words.len() == 0) {
        return None;
    }

    Some(words.len())
}

pub fn common_word_finder(contents: String) -> Option<HashMap<String, i32>> {
    // Split the contents into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    // Creates new hashmap to store the word counts (Format: Word : Count*)
    let mut word_map:HashMap<String, i32> = HashMap::new();

    // Lops trough words in the text file
    for word in words.iter() {
        let word_string = word.to_string();
        // Check if the word is already in the map
        if(word_map.contains_key(&word_string)) {
            let count = word_map.get_mut(&word_string).unwrap();
            *count += 1;
        } else {
            word_map.insert(word_string, 1);
        }
    }

    if(word_map.len() == 0) {
        return None;
    }

    Some(word_map)
}

pub fn concorde_finder(contents : String, min: usize, max: usize) -> Option<HashMap<String, usize>> {
    // // Split the contents into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    let mut con = HashMap::new();

    for (index, word) in words.iter().enumerate() {
        if(index > min) {
            for concorde_word in words.clone().iter().take(index + max).skip(index-min){
                *con.entry(concorde_word.to_lowercase()).or_insert(0) += 1;
            }
        }
    }
    if(con.len() == 0) {
        return None;
    }
    Some(con)
}