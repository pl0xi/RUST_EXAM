use std::collections::HashMap;
use crate::errors::TextAnalysisError;

pub fn count_words(contents: &str) -> Result<Option<usize>, TextAnalysisError> {
    // Split the String into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    // Count the words
    if words.is_empty() {
        return Ok(None);
    }

    Ok(Some(words.len()))
}

pub fn common_word_finder(contents: &str) -> Result<Option<HashMap<String, i32>>, TextAnalysisError> {
    // Split the contents into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    // Creates new hashmap to store the word counts (Format: Word : Count*)
    let mut word_map:HashMap<String, i32> = HashMap::new();

    // Lops trough words in the text file
    for word in words.iter() {
        let word_string = word.to_string();
        // Check if the word is already in the map
        if word_map.contains_key(&word_string) {
            let count = word_map.get_mut(&word_string).unwrap();
            *count += 1;
        } else {
            word_map.insert(word_string, 1);
        }
    }

    if word_map.is_empty() {
        return Ok(None);
    }

    Ok(Some(word_map))
}

pub fn concorde_finder(contents: &str, min: usize, max: usize) -> Result<Option<HashMap<String, usize>>, TextAnalysisError> {
    // Split the contents into words
    let words: Vec<&str> = contents.split_whitespace().collect();

    let mut concordance = HashMap::new();

    for (index, word) in words.iter().enumerate() {
        if index >= min {
            for concorde_word in words.iter().take(index + max).skip(index - min) {
                *concordance.entry(concorde_word.to_lowercase().to_string()).or_insert(0) += 1;
            }
        }
    }

    if concordance.is_empty() {
        return Ok(None);
    }

    Ok(Some(concordance))
}