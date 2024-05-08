mod text_analysis;

use std::env;
use std::fs;
use text_analysis::count_words;
use text_analysis::common_word_finder;

fn main() {

    // File path to the text file
    let file_path = "test.txt";

    // Read the contents of the file
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let count_result = count_words(contents.clone());

    let count = match count_result {
        Ok(count) => count,
        Err(err) => {
            eprintln!("Error counting words: {}", err);
            return;
        }
    };

    let common_words_result = common_word_finder(contents.clone());

    println!("Contents: \n{contents}");
    println!("Count: {count}");
    println!("Common Words: {:?}", common_words_result);
}