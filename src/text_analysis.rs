use std::io;
use std::collections::HashMap;
pub fn count_words(contents: String) -> io::Result<usize> {
    // Split the String into words
    let words: Vec<&str> = contents.split_whitespace().collect();

  /*  // Check for any non-whitespace characters if found
    if contents.chars().any(|c| !c.is_whitespace()) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid characters found"));
    }
*/

    // Count the words
    let count = words.len();

    Ok(count)
}

pub fn common_word_finder(contents: String) -> io::Result<HashMap<String, i32>> {
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

    Ok(word_map)
}