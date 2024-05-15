use std::collections::HashMap;
use std::sync::Arc;
use crate::errors::TextAnalysisError;

pub type TextAnalysisResultType<T> = Result<T, TextAnalysisError>;

pub struct CountWords {
    pub contents: Arc<String>
}

impl CountWords {
    pub fn new (contents: Arc<String>) -> Self {
        CountWords {contents}
    }
}

pub struct CommonWordFinder {
    pub contents: Arc<String>
}

impl CommonWordFinder {
    pub fn new (contents: Arc<String>) -> Self {
        CommonWordFinder {contents}
    }
}

pub struct ConcordanceFinder {
    pub contents: Arc<String>,
    pub min: usize,
    pub max: usize
}
impl ConcordanceFinder {
    pub fn new (contents: Arc<String>, min: usize, max: usize) -> Self {
        ConcordanceFinder {contents, min, max}
    }
}

pub trait TextAnalysis<T> {
    fn get_result(&self) -> TextAnalysisResultType<T>;
}

// This function uses a string slice, which is a borrowed reference to a String.
// It is used to pass data without transferring ownership.
// String slice is indicated with &str.
impl TextAnalysis<Option<String>> for CountWords {
    fn get_result(&self) -> TextAnalysisResultType<Option<String>> {
        let words: Vec<&str> = self.contents.split_whitespace().collect();

        // Count the words
        if words.is_empty() {
            return Ok(None);
        }

        Ok(Some(words.len().to_string()))
    }
}

impl TextAnalysis<Option<HashMap<String, i32>>> for CommonWordFinder {
    fn get_result(&self) -> TextAnalysisResultType<Option<HashMap<String, i32>>> {
        // Split the contents into words
        let words: Vec<&str> = self.contents.split_whitespace().collect();

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
}

impl TextAnalysis<Option<HashMap<String, usize>>> for ConcordanceFinder {
    fn get_result(&self) -> TextAnalysisResultType<Option<HashMap<String, usize>>> {
        let words: Vec<&str> = self.contents.split_whitespace().collect();

        let mut concordance = HashMap::new();

        for (index, _word) in words.iter().enumerate() {
            if index >= self.min {
                for concorde_word in words.iter().take(index + self.max).skip(index - self.min) {
                    *concordance.entry(concorde_word.to_lowercase().to_string()).or_insert(0) += 1;
                }
            }
        }

        if concordance.is_empty() {
            return Ok(None);
        }

        Ok(Some(concordance))
    }
}