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
            Err(TextAnalysisError::QuitCommand) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn perform_action (input: &str) -> Result<(), TextAnalysisError> {
    // File path to the text file
    let file_path = "test.txt";

    // The ownership of the String is transferred to 'contents'
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
            // count_words borrows contents, without taking ownership of the data.
            // This is done with &contents.
            // The data is therefore accessible to other parts of the program.
            let count_result = count_words(&contents)?;
            let count = match count_result {
                Some(count) => count,
                None => {
                    println!("No words found");
                    return Ok(());
                }
            };
            println!("Count: {count}");
        }
        "common" => {
            // common_word_finder borrows contents.
            let common_words_result = common_word_finder(&contents)?;
            match common_words_result {
                Some(common_words) => println!("Common Words: {:?}", common_words),
                None => { println!("No common words found")
                }
            }

        }
        "concorde" => {
            // concorde_finder borrows contents.
            match concorde_finder(&contents, 2, 2) {
                Ok(Some(concorde_result)) => {
                    for (word, count) in concorde_result.iter() {
                        println!("{}: {}", word, count)
                    }
                }
                Ok(None) => {
                    println!("Concordance is empty");
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }
        "quit" | "q" => {
            println!("Shutting down.");
            return Err(TextAnalysisError::QuitCommand);
        }

        _ => println!("Unknown command"),
    }
    Ok(())
}

