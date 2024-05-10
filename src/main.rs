mod text_analysis;
mod errors;

use std::io;
use std::fs;
use std::io::Read;
use text_analysis::count_words;
use text_analysis::common_word_finder;
use text_analysis::concorde_finder;
use crate::errors::TextAnalysisError;

fn main() {
    println!("Enter a command (type 'quit' or 'q' to exit):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match perform_action(&input) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

fn perform_action (input: &str) -> Result<(), TextAnalysisError> {
    // File path to the text file
    let file_path = "test.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return Err(TextAnalysisError::FileReadError);
        }
    };

    match input.trim().to_lowercase().as_str() {
        "contents" => {
            // Read the contents of the file
            println!("Contents: \n{:?}", contents);
        }
        "count" => {
            let count_result = count_words(contents.clone());
            let count = match count_result {
                Ok(count) => count,
                Err(err) => {
                    eprintln!("Error counting words: {}", err);
                    return (Err(TextAnalysisError::WordCountError));
                }
            };
            println!("Count: {count}");
        }
        "common" => {
            let common_words_result = common_word_finder(contents.clone());
            match common_words_result {
                Ok(common_words) => println!("Common Words: {:?}", common_words),
                Err(err) => {
                    eprintln!("Error finding common words: {}", err);
                    return Err(TextAnalysisError::CommonWordError);
                }
            }

        }
        "concorde" => {
            let concorde_result = concorde_finder(contents.clone(), 2, 2);
            match concorde_result {
                Ok(concorde) => {
                    for (word, count) in concorde.iter() {
                        println!("{}: {}", word, count)
                    }
                }
                Err(err) => {
                    eprintln!("Error finding concordance: {}", err);
                    return Err(TextAnalysisError::ConcordanceError);
                }
            }
        }
        "quit" | "q" => {
            println!("Shutting down.");
            return Err(TextAnalysisError::FileReadError);
        }

        _ => println!("Unknown command"),
    }
    Ok(())
}

