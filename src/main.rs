mod text_analysis;
mod errors;

use std::io;
use std::fs;
use text_analysis::{CountWords, CommonWordFinder, ConcordanceFinder};
use crate::errors::TextAnalysisError;
use crate::text_analysis::{TextAnalysis};

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
            let count_word_fn = CountWords {
                contents
            };
            let count_result = count_word_fn.get_result();

            let count = match count_result {
                Ok(Some(count)) => count,
                Ok(None) => {
                    println!("No words found");
                    return Ok(());
                },
                Err(_err) => return Err(TextAnalysisError::WordCountError)
            };
            println!("Count: {count}");
        }
        "common" => {
            // common_word_finder borrows contents.
            let common_words_fn = CommonWordFinder {
                contents
            };
            let common_words_result = common_words_fn.get_result();
            match common_words_result {
                Ok(Some(common_words)) => println!("Common Words: {:?}", common_words),
                Ok(None) =>  println!("No common words found"),
                Err(_err) => return Err(TextAnalysisError::CommonWordError)
            }

        }
        "concorde" => {
            // concorde_finder borrows contents.
            let concorde_fn = ConcordanceFinder {
                contents,
                min: 2,
                max: 2
            };

            let concorde_finder_result = concorde_fn.get_result();
            match concorde_finder_result{
                Ok(Some(concorde_result)) => {
                    for (word, count) in concorde_result.iter() {
                        println!("{}: {}", word, count)
                    }
                }
                Ok(None) => {
                    println!("Concordance is empty");
                }
                Err(err) => return Err(TextAnalysisError::ConcordanceError)
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

